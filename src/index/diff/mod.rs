use crate::{Change, Index};
use bstr::ByteSlice;
use gix::prelude::ObjectIdExt;
use gix::traverse::commit::simple::CommitTimeOrder;
use std::sync::atomic::AtomicBool;

mod delegate;
mod github;

use delegate::Delegate;

/// The order we maintain for the produced changes.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Order {
    /// Compare provided trees or commits without applying any other logic, with the order being influenced by
    /// factors like hashmaps.
    ///
    /// The benefit is mode is the optimal performance as only one diff is created.
    ImplementationDefined,
    /// If the provided revisions are commits, single step through the history that connects them to maintain
    /// the order in which changes were submitted to the crates-index for all user-defined changes.
    ///
    /// Admin changes are still implementation defined, but typically involve only deletions.
    ///
    /// The shortcomings of this approach is that each pair of commits has to be diffed individually, increasing
    /// the amount of work linearly.
    AsInCratesIndex,
}

/// The error returned by methods dealing with obtaining index changes.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Couldn't update marker reference")]
    ReferenceEdit(#[from] gix::reference::edit::Error),
    #[error("Failed to parse rev-spec to determine which revisions to diff")]
    RevParse(#[from] gix::revision::spec::parse::Error),
    #[error(transparent)]
    DiffRewrites(#[from] gix::diff::new_rewrites::Error),
    #[error("Couldn't find blob that showed up when diffing trees")]
    FindObject(#[from] gix::object::find::existing::Error),
    #[error("Couldn't get the tree of a commit for diffing purposes")]
    PeelToTree(#[from] gix::object::peel::to_kind::Error),
    #[error("Failed to diff two trees to find changed crates")]
    Diff(#[from] gix::diff::options::init::Error),
    #[error(transparent)]
    DiffForEach(#[from] gix::object::tree::diff::for_each::Error),
    #[error("Failed to decode {line:?} in file {file_name:?} as crate version")]
    VersionDecode {
        source: serde_json::Error,
        file_name: bstr::BString,
        line: bstr::BString,
    },
    #[error(transparent)]
    FindRemote(#[from] gix::remote::find::existing::Error),
    #[error(transparent)]
    FindReference(#[from] gix::reference::find::existing::Error),
    #[error(transparent)]
    Connect(#[from] gix::remote::connect::Error),
    #[error(transparent)]
    PrepareFetch(#[from] gix::remote::fetch::prepare::Error),
    #[error(transparent)]
    Fetch(#[from] gix::remote::fetch::Error),
    #[error(transparent)]
    InitAnonymousRemote(#[from] gix::remote::init::Error),
    #[error("Could not find local tracking branch for remote branch {name:?} in any of {} fetched refs", mappings.len()
    )]
    NoMatchingBranch {
        name: String,
        mappings: Vec<gix::remote::fetch::Mapping>,
    },
    #[error("Error when fetching GitHub fastpath.")]
    GithubFetch(#[from] reqwest::Error),
}

/// Find changes without modifying the underling repository
impl Index {
    /// As `peek_changes_with_options()`, but without the options.
    pub fn peek_changes(&self) -> Result<(Vec<Change>, gix::hash::ObjectId), Error> {
        self.peek_changes_with_options(
            gix::progress::Discard,
            &AtomicBool::default(),
            Order::ImplementationDefined,
        )
    }

    /// As `peek_changes()` but provides changes similar to those in the crates index.
    pub fn peek_changes_ordered(&self) -> Result<(Vec<Change>, gix::hash::ObjectId), Error> {
        self.peek_changes_with_options(
            gix::progress::Discard,
            &AtomicBool::default(),
            Order::AsInCratesIndex,
        )
    }

    /// Return all [`Change`]s that are observed between the last time `peek_changes*(…)` was called
    /// and the latest state of the `crates.io` index repository, which is obtained by fetching
    /// the remote called `origin` or whatever is configured for the current `HEAD` branch and lastly
    /// what it should be based on knowledge about he crates index.
    /// The [`Self::last_seen_reference()`] will not be created or updated.
    /// The second field in the returned tuple is the commit object to which the changes were provided.
    /// If one would set the [`Self::last_seen_reference()`] to that object, the effect is exactly the same
    /// as if [`Self::fetch_changes()`] had been called.
    ///
    /// The `progress` and `should_interrupt` parameters are used to provide progress for fetches and allow
    /// these operations to be interrupted gracefully.
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
        mut progress: P,
        should_interrupt: &AtomicBool,
        order: Order,
    ) -> Result<(Vec<Change>, gix::hash::ObjectId), Error>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
    {
        let repo = &self.repo;
        let from = repo
            .find_reference(self.seen_ref_name)
            .ok()
            .and_then(|r| r.try_id().map(|id| id.detach()))
            .unwrap_or_else(|| gix::hash::ObjectId::empty_tree(repo.object_hash()));
        let to = {
            let mut remote = self
                .remote_name
                .as_deref()
                .and_then(|name| {
                    self.repo.find_remote(name.as_bstr()).ok().or_else(|| {
                        self.repo
                            .head()
                            .ok()
                            .and_then(|head| {
                                head.into_remote(gix::remote::Direction::Fetch)
                                    .and_then(|r| r.ok())
                            })
                            .or_else(|| {
                                self.repo
                                    .find_default_remote(gix::remote::Direction::Fetch)
                                    .and_then(|r| r.ok())
                            })
                    })
                })
                .map(Ok)
                .unwrap_or_else(|| {
                    self.repo
                        .head()?
                        .into_remote(gix::remote::Direction::Fetch)
                        .map(|r| r.map_err(Error::from))
                        .or_else(|| {
                            self.repo
                                .find_default_remote(gix::remote::Direction::Fetch)
                                .map(|r| r.map_err(Error::from))
                        })
                        .unwrap_or_else(|| {
                            self.repo
                                .remote_at("https://github.com/rust-lang/crates.io-index")
                                .map_err(Into::into)
                        })
                })?;
            if remote.refspecs(gix::remote::Direction::Fetch).is_empty() {
                let spec = format!(
                    "+refs/heads/{branch}:refs/remotes/{remote}/{branch}",
                    remote = self
                        .remote_name
                        .as_ref()
                        .map(|n| n.as_bstr())
                        .unwrap_or("origin".into()),
                    branch = self.branch_name,
                );
                remote
                    .replace_refspecs(Some(spec.as_str()), gix::remote::Direction::Fetch)
                    .expect("valid statically known refspec");
            }

            let (url, _) = remote.sanitized_url_and_version(gix::remote::Direction::Fetch)?;
            if matches!(
                github::has_changes(&url, &from, self.branch_name)?,
                github::FastPath::UpToDate
            ) {
                from.clone()
            } else {
                let res: gix::remote::fetch::Outcome = remote
                    .connect(gix::remote::Direction::Fetch)?
                    .prepare_fetch(&mut progress, Default::default())?
                    .receive(&mut progress, should_interrupt)?;
                let branch_name = format!("refs/heads/{}", self.branch_name);
                let local_tracking = res
                    .ref_map
                    .mappings
                    .iter()
                    .find_map(|m| match &m.remote {
                        gix::remote::fetch::Source::Ref(r) => (r.unpack().0 == branch_name)
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
            }
        };

        Ok((
            match order {
                Order::ImplementationDefined => self.changes_between_commits(from, to)?,
                Order::AsInCratesIndex => self.changes_between_ancestor_commits(from, to)?.0,
            },
            to,
        ))
    }

    /// Similar to [`Self::changes()`], but requires `from` and `to` objects to be provided. They may point
    /// to either `Commit`s or `Tree`s.
    ///
    /// # Returns
    ///
    /// A list of atomic changes that were performed on the index
    /// between the two revisions.
    ///
    /// # Grouping and Ordering
    ///
    /// The changes are grouped by the crate they belong to.
    /// The order of the changes for each crate is **deterministic** as they are ordered by line number, ascending.
    /// The order of crates is **non-deterministic**.
    ///
    /// If a specific order is required, the changes must be sorted by the caller.
    pub fn changes_between_commits(
        &self,
        from: impl Into<gix::hash::ObjectId>,
        to: impl Into<gix::hash::ObjectId>,
    ) -> Result<Vec<Change>, Error> {
        let into_tree = |id: gix::hash::ObjectId| -> Result<gix::Tree<'_>, Error> {
            Ok(id
                .attach(&self.repo)
                .object()?
                .peel_to_kind(gix::object::Kind::Tree)?
                .into_tree())
        };
        let from = into_tree(from.into())?;
        let to = into_tree(to.into())?;
        let mut delegate = Delegate::default();
        from.changes()?
            .options(|opts| {
                opts.track_rewrites(None).track_filename();
            })
            .for_each_to_obtain_tree(&to, |change| delegate.handle(change))?;
        delegate.into_result()
    }

    /// Similar to [`Self::changes()`], but requires `ancestor_commit` and `current_commit` objects to be provided
    /// with `ancestor_commit` being in the ancestry of `current_commit`.
    ///
    /// If the invariants regarding `ancestor_commit` and `current_commit` are not upheld, we fallback
    /// to `changes_between_commits()` which doesn't have such restrictions.
    /// This can happen if the crates-index was squashed for instance.
    ///
    /// # Returns
    ///
    /// A list of atomic changes that were performed on the index
    /// between the two revisions, but looking at it one commit at a time, along with the `Order`
    /// that the changes are actually in in case one of the invariants wasn't met.
    ///
    /// # Grouping and Ordering
    ///
    /// Note that the order of the changes for each crate is **deterministic**, should they happen within one commit,
    /// as the ordering is imposed to be by line number, ascending.
    /// Typically one commit does not span multiple crates, but if it does, for instance when rollups happen,
    /// then the order of crates is also **non-deterministic**.
    ///
    pub fn changes_between_ancestor_commits(
        &self,
        ancestor_commit: impl Into<gix::hash::ObjectId>,
        current_commit: impl Into<gix::hash::ObjectId>,
    ) -> Result<(Vec<Change>, Order), Error> {
        let from_commit = ancestor_commit.into();
        let to_commit = current_commit.into();
        match self.commit_ancestry(from_commit, to_commit) {
            Some(commits) => {
                let mut changes = Vec::new();
                for from_to in commits.windows(2) {
                    let from = from_to[0];
                    let to = from_to[1];
                    changes.extend(self.changes_between_commits(from, to)?);
                }
                Ok((changes, Order::AsInCratesIndex))
            }
            None => self
                .changes_between_commits(from_commit, to_commit)
                .map(|c| (c, Order::ImplementationDefined)),
        }
    }

    /// Return a list of commits like `from_commit..=to_commits`.
    fn commit_ancestry(
        &self,
        ancestor_commit: gix::hash::ObjectId,
        current_commit: gix::hash::ObjectId,
    ) -> Option<Vec<gix::hash::ObjectId>> {
        let seconds = ancestor_commit
            .attach(&self.repo)
            .object()
            .ok()?
            .try_into_commit()
            .ok()?
            .committer()
            .ok()?
            .time
            .seconds;
        let mut commits = current_commit
            .attach(&self.repo)
            .ancestors()
            .sorting(gix::revision::walk::Sorting::ByCommitTimeCutoff {
                seconds,
                order: CommitTimeOrder::NewestFirst,
            })
            .first_parent_only()
            .all()
            .ok()?
            .map(|c| c.map(|c| c.id))
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        commits.reverse();
        if *commits.first()? != ancestor_commit {
            // try harder, commit resolution is just a second.
            let pos = commits.iter().position(|c| *c == ancestor_commit)?;
            commits = commits[pos..].into();
        }
        assert_eq!(
            commits[commits.len() - 1],
            current_commit,
            "the iterator includes the tips"
        );
        Some(commits)
    }
}

/// Find changes while changing the underlying repository in one way or another.
impl Index {
    /// As `fetch_changes_with_options()`, but without the options.
    pub fn fetch_changes(&self) -> Result<Vec<Change>, Error> {
        self.fetch_changes_with_options(
            gix::progress::Discard,
            &AtomicBool::default(),
            Order::ImplementationDefined,
        )
    }

    /// As `fetch_changes()`, but returns an ordered result.
    pub fn fetch_changes_ordered(&self) -> Result<Vec<Change>, Error> {
        self.fetch_changes_with_options(
            gix::progress::Discard,
            &AtomicBool::default(),
            Order::AsInCratesIndex,
        )
    }

    /// Return all [`Change`]s that are observed between the last time this method was called
    /// and the latest state of the `crates.io` index repository, which is obtained by fetching
    /// the remote called `origin`.
    /// The [`Self::last_seen_reference()`] will be created or adjusted to point to the latest fetched
    /// state, which causes this method to have a different result each time it is called.
    ///
    /// The `progress` and `should_interrupt` parameters are used to provide progress for fetches and allow
    /// these operations to be interrupted gracefully.
    ///
    /// `order` configures how changes should be ordered.
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
        order: Order,
    ) -> Result<Vec<Change>, Error>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
    {
        let (changes, to) = self.peek_changes_with_options(progress, should_interrupt, order)?;
        self.set_last_seen_reference(to)?;
        Ok(changes)
    }

    /// Set the last seen reference to the given Oid. It will be created if it does not yet exists.
    pub fn set_last_seen_reference(&self, to: gix::hash::ObjectId) -> Result<(), Error> {
        let repo = self.repository();
        repo.reference(
            self.seen_ref_name,
            to,
            gix::refs::transaction::PreviousValue::Any,
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
    ///
    /// # Grouping and Ordering
    ///
    /// The changes are grouped by the crate they belong to.
    /// The order of the changes for each crate is **deterministic** as they are ordered by line number, ascending.
    /// The order of crates is also **non-deterministic**.
    ///
    /// If a specific order is required, the changes must be sorted by the caller.
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
