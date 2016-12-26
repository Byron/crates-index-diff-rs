use super::CrateVersion;
use std::path::Path;
use rustc_serialize::json::Json;

use git2::build::RepoBuilder;
use git2::{Object, Oid, Reference, Delta, DiffFormat, ObjectType, Tree, Repository, ErrorClass,
           Error as GitError};
use std::str;

static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";
static LAST_SEEN_REFNAME: &'static str = "refs/heads/crates-index-diff_last-seen";
static EMPTY_TREE_HASH: &'static str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
static LINE_ADDED_INDICATOR: char = '+';

/// A wrapper for a repository of the crates.io index.
pub struct Index {
    pub seen_ref_name: &'static str,
    repo: Repository,
}

impl Index {
    pub fn repository(&self) -> &Repository {
        &self.repo
    }

    pub fn last_seen_reference(&self) -> Result<Reference, GitError> {
        self.repo.find_reference(self.seen_ref_name)
    }

    pub fn from_path_or_cloned<P>(path: P) -> Result<Index, GitError>
        where P: AsRef<Path>
    {
        let repo =
            Repository::open(path.as_ref()).or_else(|err| if err.class() == ErrorClass::Repository {
                    RepoBuilder::new().bare(true).clone(INDEX_GIT_URL, path.as_ref())
                } else {
                    Err(err)
                })?;

        Ok(Index {
            repo: repo,
            seen_ref_name: LAST_SEEN_REFNAME,
        })
    }

    pub fn fetch_changes(&self) -> Result<Vec<CrateVersion>, GitError> {
        let from = self.last_seen_reference()
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
            let latest_fetched_commit_oid = self.repo.refname_to_id("refs/remotes/origin/master")?;
            self.last_seen_reference()
                .and_then(|mut seen_ref| {
                    seen_ref.set_target(latest_fetched_commit_oid,
                                        "updating seen-ref head to latest fetched commit")
                })
                .or_else(|_err| {
                    self.repo
                        .reference(self.seen_ref_name,
                                   latest_fetched_commit_oid,
                                   true,
                                   "creating seen-ref at latest fetched commit")
                })?;
            latest_fetched_commit_oid
        };
        self.changes_from_objects(&self.repo.find_object(from, None)?,
                                  &self.repo.find_object(to, None)?)
    }

    pub fn changes<S1, S2>(&self, from: S1, to: S2) -> Result<Vec<CrateVersion>, GitError>
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        self.changes_from_objects(&self.repo.revparse_single(from.as_ref())?,
                                  &self.repo.revparse_single(to.as_ref())?)
    }

    pub fn changes_from_objects(&self, from: &Object, to: &Object) -> Result<Vec<CrateVersion>, GitError> {
        fn into_tree<'a>(repo: &'a Repository, obj: &Object) -> Result<Tree<'a>, GitError> {
            repo.find_tree(match obj.kind() {
                Some(ObjectType::Commit)
                    => obj.as_commit().expect("object of kind commit yields commit").tree_id(),
                _ => /* let it possibly fail later */ obj.id()
            })
        }
        let diff = self.repo
            .diff_tree_to_tree(Some(&into_tree(&self.repo, from)?),
                               Some(&into_tree(&self.repo, to)?),
                               None)?;
        let mut res = Vec::new();
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

                if let Some(c) = Json::from_str(content)
                    .ok()
                    .and_then(|json| CrateVersion::from_crates_diff_json(json).ok()) {
                    res.push(c)
                }
                return true;
            })
            .map(|_| res)
    }
}
