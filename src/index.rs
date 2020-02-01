use super::CrateVersion;
use serde_json;
use std::path::Path;

use git2::build::RepoBuilder;
use git2::{
    Delta, DiffFormat, Error as GitError, ErrorClass, Object, ObjectType, Oid, Reference,
    Repository, Tree,
};
use std::str;

static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";
static LAST_SEEN_REFNAME: &'static str = "refs/heads/crates-index-diff_last-seen";
static EMPTY_TREE_HASH: &'static str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
static LINE_ADDED_INDICATOR: char = '+';

/// A wrapper for a repository of the crates.io index.
pub struct Index {
    /// The name and path of the reference used to keep track of the last seen state of the
    /// crates.io repository. The default value is `refs/heads/crates-index-diff_last-seen`.
    pub seen_ref_name: &'static str,
    /// The crates.io repository.
    repo: Repository,
}

impl Index {
    /// Return the crates.io repository.
    pub fn repository(&self) -> &Repository {
        &self.repo
    }

    /// Return the reference pointing to the state we have seen after calling `fetch_changes()`.
    pub fn last_seen_reference(&self) -> Result<Reference, GitError> {
        self.repo.find_reference(self.seen_ref_name)
    }

    /// Return a new `Index` instance from the given `path`, which should contain a bare or non-bare
    /// clone of the `crates.io` index.
    /// If the directory does not contain the repository or does not exist, it will be cloned from
    /// the official location automatically (with complete history).
    pub fn from_path_or_cloned<P>(path: P) -> Result<Index, GitError>
    where
        P: AsRef<Path>,
    {
        let repo = Repository::open(path.as_ref()).or_else(|err| {
            if err.class() == ErrorClass::Repository {
                RepoBuilder::new()
                    .bare(true)
                    .clone(INDEX_GIT_URL, path.as_ref())
            } else {
                Err(err)
            }
        })?;

        Ok(Index {
            repo: repo,
            seen_ref_name: LAST_SEEN_REFNAME,
        })
    }

    /// Return all `CrateVersion`s that are observed between the last time this method was called
    /// and the latest state of the `crates.io` index repository, which is obtained by fetching
    /// the remote called `origin`.
    /// The `last_seen_reference()` will be created or adjusted to point to the latest fetched
    /// state, which causes this method to have a different result each time it is called.
    pub fn fetch_changes(&self) -> Result<Vec<CrateVersion>, GitError> {
        let from = self
            .last_seen_reference()
            .and_then(|r| {
                r.target().ok_or_else(|| {
                    GitError::from_str("last-seen reference did not have a valid target")
                })
            })
            .or_else(|_| Oid::from_str(EMPTY_TREE_HASH))?;
        let to = {
            self.repo
                .find_remote("origin")
                .and_then(|mut r| r.fetch(&["refs/heads/*:refs/remotes/origin/*"], None, None))?;
            let latest_fetched_commit_oid =
                self.repo.refname_to_id("refs/remotes/origin/master")?;
            self.last_seen_reference()
                .and_then(|mut seen_ref| {
                    seen_ref.set_target(
                        latest_fetched_commit_oid,
                        "updating seen-ref head to latest fetched commit",
                    )
                })
                .or_else(|_err| {
                    self.repo.reference(
                        self.seen_ref_name,
                        latest_fetched_commit_oid,
                        true,
                        "creating seen-ref at latest fetched commit",
                    )
                })?;
            latest_fetched_commit_oid
        };
        self.changes_from_objects(
            &self.repo.find_object(from, None)?,
            &self.repo.find_object(to, None)?,
        )
    }

    /// Return all `CreateVersion`s observed between `from` and `to`. Both parameter are ref-specs
    /// pointing to either a commit or a tree.
    /// Learn more about specifying revisions
    /// in the
    /// [official documentation](https://www.kernel.org/pub/software/scm/git/docs/gitrevisions.html)
    pub fn changes<S1, S2>(&self, from: S1, to: S2) -> Result<Vec<CrateVersion>, GitError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        self.changes_from_objects(
            &self.repo.revparse_single(from.as_ref())?,
            &self.repo.revparse_single(to.as_ref())?,
        )
    }

    /// Similar to `changes()`, but requires `from` and `to` objects to be provided. They may point
    /// to either `Commit`s or `Tree`s.
    pub fn changes_from_objects(
        &self,
        from: &Object,
        to: &Object,
    ) -> Result<Vec<CrateVersion>, GitError> {
        fn into_tree<'a>(repo: &'a Repository, obj: &Object) -> Result<Tree<'a>, GitError> {
            repo.find_tree(match obj.kind() {
                Some(ObjectType::Commit) => obj
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
        let diff = self.repo.diff_tree_to_tree(
            Some(&into_tree(&self.repo, from)?),
            Some(&into_tree(&self.repo, to)?),
            None,
        )?;
        let mut res: Vec<CrateVersion> = Vec::new();
        diff.print(DiffFormat::Patch, |delta, _, diffline| {
            if diffline.origin() != LINE_ADDED_INDICATOR {
                return true;
            }

            if !match delta.status() {
                Delta::Added | Delta::Modified => true,
                _ => false,
            } {
                return true;
            }

            let content = match str::from_utf8(diffline.content()) {
                Ok(c) => c,
                Err(_) => return true,
            };

            if let Ok(c) = serde_json::from_str(content) {
                res.push(c)
            }
            return true;
        })
        .map(|_| res)
    }
}
