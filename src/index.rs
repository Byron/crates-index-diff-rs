use std::path::Path;
use rustc_serialize::json::{self, Json};

use git2::build::RepoBuilder;
use git2::{Object, BranchType, Oid, Branch, Reference, Delta, DiffFormat, ObjectType, Tree,
           Repository, ErrorClass, Error as GitError};
use std::str;

static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";
static LAST_SEEN_REFNAME: &'static str = "crates-index-diff_last-seen";
static EMPTY_TREE_HASH: &'static str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

pub struct Index {
    pub seen_ref_name: &'static str,
    repo: Repository,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ChangeType {
    Added,
    Yanked,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Crate {
    pub name: String,
    pub state: ChangeType,
    pub version: String,
}

#[derive(PartialOrd, PartialEq, Debug)]
enum CrateDecodeError {
    InvalidTopology { json: Json },
    MissingFieldError {
        object: json::Object,
        field: &'static str,
    },
    StringExpected { json: Json },
    BoolExpected { json: Json },
}

use self::CrateDecodeError::*;

impl Crate {
    fn from_json(value: Json) -> Result<Crate, CrateDecodeError> {
        fn extract<'a>(o: &'a json::Object,
                       field: &'static str)
                       -> Result<&'a Json, CrateDecodeError> {
            o.get(field).ok_or_else(|| {
                MissingFieldError {
                    object: o.clone(),
                    field: field,
                }
            })
        }

        fn into_string(value: &Json) -> Result<String, CrateDecodeError> {
            value.as_string()
                .ok_or_else(|| StringExpected { json: value.clone() })
                .map(Into::into)
        }

        fn into_bool(value: &Json) -> Result<bool, CrateDecodeError> {
            value.as_boolean()
                .ok_or_else(|| BoolExpected { json: value.clone() })
                .map(Into::into)
        }

        value.as_object().ok_or_else(|| InvalidTopology { json: value.clone() }).and_then(|o| {
            extract(o, "name")
                .and_then(into_string)
                .and_then(|name| {
                    extract(o, "vers")
                        .and_then(into_string)
                        .and_then(|version| {
                            extract(o, "yanked").and_then(into_bool).map(|yanked| {
                                Crate {
                                    name: name,
                                    state: if yanked {
                                        ChangeType::Yanked
                                    } else {
                                        ChangeType::Added
                                    },
                                    version: version,
                                }
                            })
                        })
                })
        })
    }
}

impl Index {
    pub fn repository(&self) -> &Repository {
        &self.repo
    }

    pub fn last_seen_reference(&self) -> Result<Reference, GitError> {
        self.repo.find_branch(self.seen_ref_name, BranchType::Local).map(Branch::into_reference)
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

    pub fn fetch_changes(&self) -> Result<Vec<Crate>, GitError> {
        let from = self.last_seen_reference()
            .and_then(|r| {
                r.target().ok_or_else(|| {
                    GitError::from_str("last-seen reference did not have a valid target")
                })
            })
            .or_else(|_| Oid::from_str(EMPTY_TREE_HASH))?;
        let to = self.repo
            .find_remote("origin")
            .and_then(|mut r| {
                r.fetch(&["refs/heads/*:refs/remotes/origin/*"], None, None)
                    .and_then(|_| {
                        self.repo
                            .refname_to_id("refs/remotes/origin/master")
                            .and_then(|oid| {
                                self.last_seen_reference()
                                    .and_then(|mut seen_ref| seen_ref.set_target(oid, ""))
                                    .or_else(|_err| {
                                        self.repo
                                            .find_commit(oid)
                                            .and_then(|commit| {
                                                self.repo
                                                    .branch(LAST_SEEN_REFNAME, &commit, true)
                                                    .map(Branch::into_reference)
                                            })
                                    })
                                    .map(|_| oid)
                            })
                    })
            })?;
        self.changes_from_objects(self.repo.find_object(from, None)?,
                                  self.repo.find_object(to, None)?)
    }

    pub fn changes<S1, S2>(&self, from: S1, to: S2) -> Result<Vec<Crate>, GitError>
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        self.changes_from_objects(self.repo.revparse_single(from.as_ref())?,
                                  self.repo.revparse_single(to.as_ref())?)
    }

    pub fn changes_from_objects(&self, from: Object, to: Object) -> Result<Vec<Crate>, GitError> {
        fn into_tree<'a>(repo: &'a Repository, obj: Object) -> Result<Tree<'a>, GitError> {
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
        diff.print(DiffFormat::Patch, |delta, _, diffline| -> bool {
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
                if diffline.origin() != '+' {
                    return true;
                }

                if let Some(c) = Json::from_str(content)
                    .ok()
                    .and_then(|json| Crate::from_json(json).ok()) {
                    res.push(c)
                }
                return true;
            })?;

        Ok(res)
    }
}
