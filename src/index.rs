use super::{Change, CrateVersion};
use std::path::Path;

use git2::{
    build::RepoBuilder, Delta, Error as GitError, ErrorClass, Object, ObjectType, Oid, Reference,
    Repository, Tree,
};
use std::str;
use crate::Index;

static INDEX_GIT_URL: &str = "https://github.com/rust-lang/crates.io-index";
static LAST_SEEN_REFNAME: &str = "refs/heads/crates-index-diff_last-seen";
static EMPTY_TREE_HASH: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
static LINE_ADDED_INDICATOR: char = '+';

/// Options for use in `Index::from_path_or_cloned_with_options`
pub struct CloneOptions<'a> {
    /// The url from which the repository should be cloned.
    pub repository_url: String,
    /// Git2 fetch options to control exactly how to clone.
    pub fetch_options: Option<git2::FetchOptions<'a>>,
}

impl<'a> Default for CloneOptions<'a> {
    fn default() -> Self {
        CloneOptions {
            repository_url: INDEX_GIT_URL.into(),
            fetch_options: None,
        }
    }
}

impl Index {
    /// Return the crates.io repository.
    pub fn repository(&self) -> &Repository {
        &self.repo
    }

    /// Return the reference pointing to the state we have seen after calling `fetch_changes()`.
    pub fn last_seen_reference(&self) -> Result<Reference<'_>, GitError> {
        self.repo.find_reference(self.seen_ref_name)
    }

    /// Return a new `Index` instance from the given `path`, which should contain a bare or non-bare
    /// clone of the `crates.io` index.
    /// If the directory does not contain the repository or does not exist, it will be cloned from
    /// the official location automatically (with complete history).
    ///
    /// An error will occour if the repository exists and the remote URL does not match the given repository URL.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crates_index_diff::{Index, index};
    ///
    /// # let path = tempdir::TempDir::new("index").unwrap();
    /// let mut options = index::CloneOptions {
    ///   repository_url: "https://github.com/rust-lang/staging.crates.io-index".into(),
    ///   ..Default::default()
    /// };
    ///
    ///
    /// let index = Index::from_path_or_cloned_with_options(path, options)?;
    /// # Ok::<(), git2::Error>(())
    /// ```
    /// Or to access a private repository, use fetch options.
    ///
    /// ```no_run
    /// use crates_index_diff::{index, Index};
    /// let fo = {
    ///     let mut fo = git2::FetchOptions::new();
    ///     fo.remote_callbacks({
    ///         let mut callbacks = git2::RemoteCallbacks::new();
    ///         callbacks.credentials(|_url, username_from_url, _allowed_types| {
    ///             git2::Cred::ssh_key_from_memory(
    ///                 username_from_url.unwrap(),
    ///                 None,
    ///                 &std::env::var("PRIVATE_KEY").unwrap(),
    ///                 None,
    ///             )
    ///         });
    ///         callbacks
    ///     });
    ///     fo
    /// };
    /// Index::from_path_or_cloned_with_options(
    ///     "index",
    ///     index::CloneOptions {
    ///         repository_url: "git@github.com:private-index/goes-here.git".into(),
    ///         fetch_options: Some(fo),
    ///     },
    /// ).unwrap();
    /// ```
    pub fn from_path_or_cloned_with_options(
        path: impl AsRef<Path>,
        CloneOptions {
            repository_url,
            fetch_options,
        }: CloneOptions<'_>,
    ) -> Result<Index, GitError> {
        let mut repo_did_exist = true;
        let repo = Repository::open(path.as_ref()).or_else(|err| {
            if err.class() == ErrorClass::Repository {
                repo_did_exist = false;
                let mut builder = RepoBuilder::new();
                if let Some(fo) = fetch_options {
                    builder.fetch_options(fo);
                }
                builder.bare(true).clone(&repository_url, path.as_ref())
            } else {
                Err(err)
            }
        })?;

        if repo_did_exist {
            let remote = repo.find_remote("origin")?;
            let actual_remote_url = remote
                .url()
                .ok_or_else(|| GitError::from_str("did not obtain URL of remote named 'origin'"))?;
            if actual_remote_url != repository_url {
                return Err(GitError::from_str(&format!(
                    "Actual 'origin' remote url {:#?} did not match desired one at {:#?}",
                    actual_remote_url, repository_url
                )));
            }
        }

        Ok(Index {
            repo,
            seen_ref_name: LAST_SEEN_REFNAME,
        })
    }

    /// Return a new `Index` instance from the given `path`, which should contain a bare or non-bare
    /// clone of the `crates.io` index.
    /// If the directory does not contain the repository or does not exist, it will be cloned from
    /// the official location automatically (with complete history).
    pub fn from_path_or_cloned(path: impl AsRef<Path>) -> Result<Index, GitError> {
        Index::from_path_or_cloned_with_options(path, CloneOptions::default())
    }

    /// As `peek_changes_with_options`, but without the options.
    pub fn peek_changes(&self) -> Result<(Vec<Change>, git2::Oid), GitError> {
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
    ) -> Result<(Vec<Change>, git2::Oid), GitError> {
        let from = self
            .last_seen_reference()
            .and_then(|r| {
                r.target().ok_or_else(|| {
                    GitError::from_str("last-seen reference did not have a valid target")
                })
            })
            .or_else(|_| Oid::from_str(EMPTY_TREE_HASH))?;
        let to = {
            self.repo.find_remote("origin").and_then(|mut r| {
                r.fetch(
                    &["refs/heads/master:refs/remotes/origin/master"],
                    options,
                    None,
                )
            })?;
            self.repo.refname_to_id("refs/remotes/origin/master")?
        };

        Ok((
            self.changes_from_objects(
                &self.repo.find_object(from, None)?,
                &self.repo.find_object(to, None)?,
            )?,
            to,
        ))
    }

    /// As `fetch_changes_with_options`, but without the options.
    pub fn fetch_changes(&self) -> Result<Vec<Change>, GitError> {
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
    ) -> Result<Vec<Change>, GitError> {
        let (changes, to) = self.peek_changes_with_options(options)?;
        self.set_last_seen_reference(to)?;
        Ok(changes)
    }

    /// Set the last seen reference to the given Oid. It will be created if it does not yet exists.
    pub fn set_last_seen_reference(&self, to: Oid) -> Result<(), GitError> {
        self.last_seen_reference()
            .and_then(|mut seen_ref| {
                seen_ref.set_target(to, "updating seen-ref head to latest fetched commit")
            })
            .or_else(|_err| {
                self.repo.reference(
                    self.seen_ref_name,
                    to,
                    true,
                    "creating seen-ref at latest fetched commit",
                )
            })?;
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
    ) -> Result<Vec<Change>, GitError> {
        self.changes_from_objects(
            &self.repo.revparse_single(from.as_ref())?,
            &self.repo.revparse_single(to.as_ref())?,
        )
    }

    /// Similar to `changes()`, but requires `from` and `to` objects to be provided. They may point
    /// to either `Commit`s or `Tree`s.
    pub fn changes_from_objects(
        &self,
        from: &Object<'_>,
        to: &Object<'_>,
    ) -> Result<Vec<Change>, GitError> {
        fn into_tree<'a>(repo: &'a Repository, obj: &Object<'_>) -> Result<Tree<'a>, GitError> {
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
        let mut changes: Vec<Change> = Vec::new();
        let mut deletes: Vec<String> = Vec::new();
        diff.foreach(
            &mut |delta, _| {
                if delta.status() == Delta::Deleted {
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
                if !matches!(delta.status(), Delta::Added | Delta::Modified) {
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

        changes.extend(deletes.iter().map(|krate| Change::Deleted(krate.clone())));
        Ok(changes)
    }
}
