use crate::{Change, Index};
use git_repository as git;
use git_repository::prelude::ObjectIdExt;
use std::sync::atomic::AtomicBool;

mod delegate;
use delegate::Delegate;

/// The error returned by methods dealing with obtaining index changes.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Couldn't update marker reference")]
    ReferenceEdit(#[from] git::reference::edit::Error),
    #[error("Failed to parse rev-spec to determine which revisions to diff")]
    RevParse(#[from] git::revision::spec::parse::Error),
    #[error("Couldn't find blob that showed up when diffing trees")]
    FindObject(#[from] git::object::find::existing::Error),
    #[error("Couldn't get the tree of a commit for diffing purposes")]
    PeelToTree(#[from] git::object::peel::to_kind::Error),
    #[error("Failed to diff two trees to find changed crates")]
    Diff(#[from] git::object::blob::diff::init::Error),
    #[error(transparent)]
    DiffForEach(#[from] git::object::tree::diff::for_each::Error),
    #[error("Failed to decode {line:?} in file {file_name:?} as crate version")]
    VersionDecode {
        source: serde_json::Error,
        file_name: bstr::BString,
        line: bstr::BString,
    },
    #[error(transparent)]
    FindRemote(#[from] git::remote::find::existing::Error),
    #[error(transparent)]
    FindReference(#[from] git::reference::find::existing::Error),
    #[error(transparent)]
    Connect(#[from] git::remote::connect::Error),
    #[error(transparent)]
    PrepareFetch(#[from] git::remote::fetch::prepare::Error),
    #[error(transparent)]
    Fetch(#[from] git::remote::fetch::Error),
    #[error(transparent)]
    InitAnonymousRemote(#[from] git::remote::init::Error),
    #[error("Could not find local tracking branch for remote branch {name:?} in any of {} fetched refs", mappings.len())]
    NoMatchingBranch {
        name: String,
        mappings: Vec<git::remote::fetch::Mapping>,
    },
}

/// Find changes without modifying the underling repository
impl Index {
    /// As `peek_changes_with_options`, but without the options.
    pub fn peek_changes(&self) -> Result<(Vec<Change>, git::hash::ObjectId), Error> {
        self.peek_changes_with_options(git::progress::Discard, &AtomicBool::default())
    }

    /// Return all `Change`s that are observed between the last time `peek_changes*(…)` was called
    /// and the latest state of the `crates.io` index repository, which is obtained by fetching
    /// the remote called `origin` or whatever is configured for the current `HEAD` branch and lastly
    /// what it should be based on knowledge about he crates index.
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
    // TODO: update this once it's clear how auto-gc works in `gitoxide`.
    pub fn peek_changes_with_options<P>(
        &self,
        progress: P,
        should_interrupt: &AtomicBool,
    ) -> Result<(Vec<Change>, git::hash::ObjectId), Error>
    where
        P: git::Progress,
        P::SubProgress: 'static,
    {
        let repo = &self.repo;
        let from = repo
            .find_reference(self.seen_ref_name)
            .ok()
            .and_then(|r| r.try_id().map(|id| id.detach()))
            .unwrap_or_else(|| git::hash::ObjectId::empty_tree(repo.object_hash()));
        let to = {
            let mut remote = self
                .remote_name
                .as_deref()
                .and_then(|name| {
                    self.repo.find_remote(name).ok().or_else(|| {
                        self.repo
                            .head()
                            .ok()
                            .and_then(|head| {
                                head.into_remote(git::remote::Direction::Fetch)
                                    .and_then(|r| r.ok())
                            })
                            .or_else(|| {
                                self.repo
                                    .find_default_remote(git::remote::Direction::Fetch)
                                    .and_then(|r| r.ok())
                            })
                    })
                })
                .map(Ok)
                .unwrap_or_else(|| {
                    self.repo
                        .head()?
                        .into_remote(git::remote::Direction::Fetch)
                        .map(|r| r.map_err(Error::from))
                        .or_else(|| {
                            self.repo
                                .find_default_remote(git::remote::Direction::Fetch)
                                .map(|r| r.map_err(Error::from))
                        })
                        .unwrap_or_else(|| {
                            self.repo
                                .remote_at("https://github.com/rust-lang/crates.io-index")
                                .map_err(Into::into)
                        })
                })?;
            if remote.refspecs(git::remote::Direction::Fetch).is_empty() {
                let spec = format!(
                    "+refs/heads/{branch}:refs/remotes/{remote}/{branch}",
                    remote = self.remote_name.as_deref().unwrap_or("origin"),
                    branch = self.branch_name,
                );
                remote
                    .replace_refspecs(Some(spec.as_str()), git::remote::Direction::Fetch)
                    .expect("valid statically known refspec");
            }
            let res: git::remote::fetch::Outcome = remote
                .connect(git::remote::Direction::Fetch, progress)?
                .prepare_fetch(Default::default())?
                .receive(should_interrupt)?;
            let branch_name = format!("refs/heads/{}", self.branch_name);
            let local_tracking = res
                .ref_map
                .mappings
                .iter()
                .find_map(|m| match &m.remote {
                    git::remote::fetch::Source::Ref(r) => (r.unpack().0 == branch_name)
                        .then_some(m.local.as_ref())
                        .flatten(),
                    _ => None,
                })
                .ok_or_else(|| Error::NoMatchingBranch {
                    name: branch_name,
                    mappings: res.ref_map.mappings.clone(),
                })?;
            self.repo
                .find_reference(local_tracking)
                .expect("local tracking branch exists if we see it here")
                .id()
                .detach()
        };

        Ok((self.changes_between_commits(from, to)?, to))
    }

    /// Similar to `changes()`, but requires `from` and `to` objects to be provided. They may point
    /// to either `Commit`s or `Tree`s.
    ///
    /// # Returns
    ///
    /// A list of atomic chanes that were performed on the index
    /// between the two revisions.
    /// The changes are grouped by the crate they belong to.
    /// The order of the changes for each crate are **non-deterministic**.
    /// The order of crates is also **non-deterministic**.
    ///
    /// If a specific order is required, the changes must be sorted by the calle
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
        let mut delegate = Delegate::default();
        from.changes()
            .track_filename()
            .for_each_to_obtain_tree(&to, |change| delegate.handle(change))?;
        delegate.into_result()
    }
}

/// Find changes while changing the underlying repository in one way or another.
impl Index {
    /// As `fetch_changes_with_options`, but without the options.
    pub fn fetch_changes(&self) -> Result<Vec<Change>, Error> {
        self.fetch_changes_with_options(git::progress::Discard, &AtomicBool::default())
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
    pub fn fetch_changes_with_options<P>(
        &self,
        progress: P,
        should_interrupt: &AtomicBool,
    ) -> Result<Vec<Change>, Error>
    where
        P: git::Progress,
        P::SubProgress: 'static,
    {
        let (changes, to) = self.peek_changes_with_options(progress, should_interrupt)?;
        self.set_last_seen_reference(to)?;
        Ok(changes)
    }

    /// Set the last seen reference to the given Oid. It will be created if it does not yet exists.
    pub fn set_last_seen_reference(&self, to: git::hash::ObjectId) -> Result<(), Error> {
        let repo = self.repository();
        repo.reference(
            self.seen_ref_name,
            to,
            git::refs::transaction::PreviousValue::Any,
            "updating seen-ref head to latest fetched commit",
        )?;
        Ok(())
    }

    /// Return all `CreateVersion`s observed between `from` and `to`. Both parameter are ref-specs
    /// pointing to either a commit or a tree.
    /// Learn more about specifying revisions
    /// in the
    /// [official documentation](https://www.kernel.org/pub/software/scm/git/docs/gitrevisions.html)
    ///
    /// # Returns
    ///
    /// A list of atomic chanes that were performed on the index
    /// between the two revisions.
    /// The changes are grouped by the crate they belong to.
    /// The order of the changes for each crate are **non-deterministic**.
    /// The order of crates is also **non-deterministic**.
    ///
    /// If a specific order is required, the changes must be sorted by the calle
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
