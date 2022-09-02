use crate::{Change, Index};
use git_repository as git;
use git_repository::prelude::{FindExt, ObjectIdExt};
use git_repository::refs::transaction::PreviousValue;
use std::convert::TryFrom;

mod delegate;
use delegate::Delegate;

/// The error returned by methods dealing with obtaining index changes.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Failed to fetch crates.io index repository")]
    Fetch(#[from] git2::Error),
    #[error("Couldn't update marker reference")]
    ReferenceEdit(#[from] git::reference::edit::Error),
    #[error("Failed to parse rev-spec to determine which revisions to diff")]
    RevParse(#[from] git::revision::spec::parse::Error),
    #[error("Couldn't find blob that showed up when diffing trees")]
    FindObject(#[from] git::object::find::existing::Error),
    #[error("Couldn't get the tree of a commit for diffing purposes")]
    PeelToTree(#[from] git::object::peel::to_kind::Error),
    #[error("Failed to diff two trees to find changed crates")]
    Diff(#[from] git::diff::tree::changes::Error),
    #[error("Failed to decode {line:?} in file {file_name:?} as crate version")]
    VersionDecode {
        source: serde_json::Error,
        file_name: bstr::BString,
        line: bstr::BString,
    },
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
            repo.find_remote(self.remote_name).and_then(|mut r| {
                r.fetch(
                    &[format!(
                        "+refs/heads/{branch}:refs/remotes/{remote}/{branch}",
                        remote = self.remote_name,
                        branch = self.branch_name,
                    )],
                    options,
                    None,
                )
            })?;
            git::hash::ObjectId::try_from(
                repo.refname_to_id(&format!(
                    "refs/remotes/{}/{}",
                    self.remote_name, self.branch_name
                ))?
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
        let file_changes =
            git::diff::tree::Changes::from(git::objs::TreeRefIter::from_bytes(&from.data))
                .needed_to_obtain(
                    git::objs::TreeRefIter::from_bytes(&to.data),
                    git::diff::tree::State::default(),
                    |id, buf| self.repo.objects.find_tree_iter(id, buf),
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
