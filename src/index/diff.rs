use crate::{Change, CrateVersion, Index};

static EMPTY_TREE_HASH: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
static LINE_ADDED_INDICATOR: char = '+';

/// Find changes without modifying the underling repository
impl Index {
    /// As `peek_changes_with_options`, but without the options.
    pub fn peek_changes(&self) -> Result<(Vec<Change>, git2::Oid), git2::Error> {
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
    ) -> Result<(Vec<Change>, git2::Oid), git2::Error> {
        let from = self
            .last_seen_reference()
            .and_then(|r| {
                r.target().ok_or_else(|| {
                    git2::Error::from_str("last-seen reference did not have a valid target")
                })
            })
            .or_else(|_| git2::Oid::from_str(EMPTY_TREE_HASH))?;
        let to = {
            self.repo.find_remote("origin").and_then(|mut r| {
                r.fetch(
                    &[format!(
                        "refs/heads/{branch}:refs/remotes/origin/{branch}",
                        branch = self.branch_name
                    )],
                    options,
                    None,
                )
            })?;
            self.repo
                .refname_to_id(&format!("refs/remotes/origin/{}", self.branch_name))?
        };

        Ok((
            self.changes_from_objects(
                &self.repo.find_object(from, None)?,
                &self.repo.find_object(to, None)?,
            )?,
            to,
        ))
    }

    /// Similar to `changes()`, but requires `from` and `to` objects to be provided. They may point
    /// to either `Commit`s or `Tree`s.
    pub fn changes_from_objects(
        &self,
        from: &git2::Object<'_>,
        to: &git2::Object<'_>,
    ) -> Result<Vec<Change>, git2::Error> {
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
        let diff = self.repo.diff_tree_to_tree(
            Some(&into_tree(&self.repo, from)?),
            Some(&into_tree(&self.repo, to)?),
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

        changes.extend(deletes.iter().map(|krate| Change::Deleted(krate.clone())));
        Ok(changes)
    }
}

/// Find changes while changing the underlying repository in one way or another.
impl Index {
    /// As `fetch_changes_with_options`, but without the options.
    pub fn fetch_changes(&self) -> Result<Vec<Change>, git2::Error> {
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
    ) -> Result<Vec<Change>, git2::Error> {
        let (changes, to) = self.peek_changes_with_options(options)?;
        self.set_last_seen_reference(to)?;
        Ok(changes)
    }

    /// Set the last seen reference to the given Oid. It will be created if it does not yet exists.
    pub fn set_last_seen_reference(&self, to: git2::Oid) -> Result<(), git2::Error> {
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
    ) -> Result<Vec<Change>, git2::Error> {
        self.changes_from_objects(
            &self.repo.revparse_single(from.as_ref())?,
            &self.repo.revparse_single(to.as_ref())?,
        )
    }
}
