use crate::{Change, CrateVersion, Index};
use git_repository as git;
use git_repository::prelude::{FindExt, ObjectIdExt, TreeIterExt};
use git_repository::refs::transaction::PreviousValue;
use std::convert::TryFrom;

static LINE_ADDED_INDICATOR: char = '+';

mod delegate;
use delegate::Delegate;

/// The error returned by methods dealing with obtaining index changes.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] git2::Error),
    #[error(transparent)]
    ReferenceEdit(#[from] git::reference::edit::Error),
    #[error(transparent)]
    RevParse(#[from] git::revision::spec::parse::Error),
    #[error(transparent)]
    FindObject(#[from] git::object::find::existing::Error),
    #[error(transparent)]
    PeelToTree(#[from] git::object::peel::to_kind::Error),
    #[error(transparent)]
    Diff(#[from] git::diff::tree::changes::Error),
    #[error(transparent)]
    VersionDecode(#[from] serde_json::Error),
}

/// Find changes without modifying the underling repository
impl Index {
    /// As `peek_changes_with_options`, but without the options.
    pub fn peek_changes(&self) -> Result<(Vec<Change>, git::hash::ObjectId), Error> {
        self.peek_changes_with_options(None)
    }

    /// Return all `Change`s that are observed between the last time `fetch_changes(…)` was called
    /// and the latest state of the `crates.io` index repository, which is obtained by fetching
    /// the remote called `origin`.
    /// The `last_seen_reference()` will not be created or updated.
    /// The second field in the returned tuple is the commit object to which the changes were provided.
    /// If one would set the `last_seen_reference()` to that object, the effect is exactly the same
    /// as if `fetch_changes(…)` had been called.
    ///
    /// # Resource Usage
    ///
    /// As this method fetches the git repository, loose objects or small packs may be created. Over time,
    /// these will accumulate and either slow down subsequent operations, or cause them to fail due to exhaustion
    /// of the maximum number of open file handles as configured with `ulimit`.
    ///
    /// Thus it is advised for the caller to run `git gc` occasionally based on their own requirements and usage patterns.
    pub fn peek_changes_with_options(
        &self,
        options: Option<&mut git2::FetchOptions<'_>>,
    ) -> Result<(Vec<Change>, git::hash::ObjectId), Error> {
        let repo = &self.repo;
        let from = repo
            .find_reference(self.seen_ref_name)
            .ok()
            .and_then(|r| r.try_id().map(|id| id.detach()))
            .unwrap_or_else(|| git::hash::ObjectId::empty_tree(repo.object_hash()));
        let to = {
            let repo = git2::Repository::open(repo.git_dir())?;
            repo.find_remote("origin").and_then(|mut r| {
                r.fetch(
                    &[format!(
                        "refs/heads/{branch}:refs/remotes/origin/{branch}",
                        branch = self.branch_name
                    )],
                    options,
                    None,
                )
            })?;
            git::hash::ObjectId::try_from(
                repo.refname_to_id(&format!("refs/remotes/origin/{}", self.branch_name))?
                    .as_bytes(),
            )
            .expect("valid oid")
        };

        Ok((self.changes_between_commits(from, to)?, to))
    }

    /// Similar to `changes()`, but requires `from` and `to` objects to be provided. They may point
    /// to either `Commit`s or `Tree`s.
    pub fn changes_between_commits(
        &self,
        from: impl Into<git::hash::ObjectId>,
        to: impl Into<git::hash::ObjectId>,
    ) -> Result<Vec<Change>, Error> {
        let repo = git2::Repository::open(self.repo.git_dir())?;
        let from = git2::Oid::from_bytes(from.into().as_slice())?;
        let to = git2::Oid::from_bytes(to.into().as_slice())?;
        fn into_tree<'a>(
            repo: &'a git2::Repository,
            obj: &git2::Object<'_>,
        ) -> Result<git2::Tree<'a>, git2::Error> {
            repo.find_tree(match obj.kind() {
                Some(git2::ObjectType::Commit) => obj
                    .as_commit()
                    .expect("object of kind commit yields commit")
                    .tree_id(),
                _ =>
                /* let it possibly fail later */
                {
                    obj.id()
                }
            })
        }
        let from = repo.find_object(from, None)?;
        let to = repo.find_object(to, None)?;
        let diff = repo.diff_tree_to_tree(
            Some(&into_tree(&repo, &from)?),
            Some(&into_tree(&repo, &to)?),
            None,
        )?;
        let mut changes: Vec<Change> = Vec::new();
        let mut deletes: Vec<String> = Vec::new();
        diff.foreach(
            &mut |delta, _| {
                if delta.status() == git2::Delta::Deleted {
                    if let Some(path) = delta.new_file().path() {
                        if let Some(file_name) = path.file_name() {
                            deletes.push(file_name.to_string_lossy().to_string());
                        }
                    }
                }
                true
            },
            None,
            None,
            Some(&mut |delta, _hunk, diffline| {
                if diffline.origin() != LINE_ADDED_INDICATOR {
                    return true;
                }
                if !matches!(delta.status(), git2::Delta::Added | git2::Delta::Modified) {
                    return true;
                }

                if let Ok(crate_version) =
                    serde_json::from_slice::<CrateVersion>(diffline.content())
                {
                    if crate_version.yanked {
                        changes.push(Change::Yanked(crate_version));
                    } else {
                        changes.push(Change::Added(crate_version));
                    }
                }
                true
            }),
        )?;

        changes.extend(deletes.iter().map(|krate| Change::Deleted {
            name: krate.clone(),
        }));
        Ok(changes)
    }

    /// Similar to `changes()`, but requires `from` and `to` objects to be provided. They may point
    /// to either `Commit`s or `Tree`s.
    pub fn changes_between_commits2(
        &mut self,
        from: impl Into<git::hash::ObjectId>,
        to: impl Into<git::hash::ObjectId>,
    ) -> Result<Vec<Change>, Error> {
        self.repo.object_cache_size_if_unset(4 * 1024 * 1024);
        let into_tree = |id: git::hash::ObjectId| -> Result<git::Tree<'_>, Error> {
            Ok(id
                .attach(&self.repo)
                .object()?
                .peel_to_kind(git::object::Kind::Tree)?
                .into_tree())
        };
        let from = into_tree(from.into())?;
        let to = into_tree(to.into())?;
        let mut delegate = Delegate::from_repo(&self.repo);
        let file_changes = git::objs::TreeRefIter::from_bytes(&from.data).changes_needed(
            git::objs::TreeRefIter::from_bytes(&to.data),
            git::diff::tree::State::default(),
            |id, buf| self.repo.objects.find_tree_iter(id, buf).ok(),
            &mut delegate,
        );
        match file_changes.err() {
            None | Some(git::diff::tree::changes::Error::Cancelled) => { /*error in delegate*/ }
            Some(err) => return Err(err.into()),
        }
        delegate.into_result()
    }
}

/// Find changes while changing the underlying repository in one way or another.
impl Index {
    /// As `fetch_changes_with_options`, but without the options.
    pub fn fetch_changes(&self) -> Result<Vec<Change>, Error> {
        self.fetch_changes_with_options(None)
    }

    /// Return all `Change`s that are observed between the last time this method was called
    /// and the latest state of the `crates.io` index repository, which is obtained by fetching
    /// the remote called `origin`.
    /// The `last_seen_reference()` will be created or adjusted to point to the latest fetched
    /// state, which causes this method to have a different result each time it is called.
    ///
    /// # Resource Usage
    ///
    /// As this method fetches the git repository, loose objects or small packs may be created. Over time,
    /// these will accumulate and either slow down subsequent operations, or cause them to fail due to exhaustion
    /// of the maximum number of open file handles as configured with `ulimit`.
    ///
    /// Thus it is advised for the caller to run `git gc` occasionally based on their own requirements and usage patterns.
    pub fn fetch_changes_with_options(
        &self,
        options: Option<&mut git2::FetchOptions<'_>>,
    ) -> Result<Vec<Change>, Error> {
        let (changes, to) = self.peek_changes_with_options(options)?;
        self.set_last_seen_reference(to)?;
        Ok(changes)
    }

    /// Set the last seen reference to the given Oid. It will be created if it does not yet exists.
    pub fn set_last_seen_reference(&self, to: git::hash::ObjectId) -> Result<(), Error> {
        let repo = self.repository();
        repo.reference(
            self.seen_ref_name,
            to,
            PreviousValue::Any,
            "updating seen-ref head to latest fetched commit",
        )?;
        Ok(())
    }

    /// Return all `CreateVersion`s observed between `from` and `to`. Both parameter are ref-specs
    /// pointing to either a commit or a tree.
    /// Learn more about specifying revisions
    /// in the
    /// [official documentation](https://www.kernel.org/pub/software/scm/git/docs/gitrevisions.html)
    pub fn changes(
        &self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> Result<Vec<Change>, Error> {
        let repo = self.repository();
        let from = repo
            .rev_parse(from.as_ref())?
            .single()
            .expect("revspec was not a range")
            .detach();
        let to = repo
            .rev_parse(to.as_ref())?
            .single()
            .expect("revspec was not a range")
            .detach();
        self.changes_between_commits(from, to)
    }
}
