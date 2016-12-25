use std::path::Path;
use rustc_serialize::json::{self, Json};

use git2::build::RepoBuilder;
use git2::{Delta, DiffFormat, DiffDelta, DiffHunk, DiffLine, ObjectType, Tree, Repository,
    ErrorClass, Error as GitError};
use std::str;

static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";

pub struct Index {
    repo: Repository,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ChangeType {
    Added,
    Yanked
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Crate {
    pub name: String,
    pub state: ChangeType,
    pub version: String,
}

#[derive(PartialOrd, PartialEq, Debug)]
enum CrateDecodeError {
    InvalidTopology {
        json: Json
    },
    MissingFieldError {
        object: json::Object,
        field: &'static str,
    },
    StringExpected {
        json: Json
    },
    BoolExpected {
        json: Json
    },
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
    pub fn at_path<P>(path: P) -> Result<Index, GitError>
        where P: AsRef<Path>
    {
        let repo =
        Repository::open(path.as_ref()).or_else(|err| if err.class() == ErrorClass::Repository {
            RepoBuilder::new().bare(true).clone(INDEX_GIT_URL, path.as_ref())
        } else {
            Err(err)
        })?;

        Ok(Index { repo: repo })
    }

    pub fn changes<S1, S2>(&self, from: S1, to: S2) -> Result<Vec<Crate>, GitError>
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        fn into_tree<S: AsRef<str>>(repo: &Repository, rev: S) -> Result<Tree, GitError> {
            repo.revparse_single(rev.as_ref()).and_then(|obj| {
                repo.find_tree(match obj.kind() {
                    Some(ObjectType::Commit) => obj.as_commit().expect("valid commit").tree_id(),
                    _ => /* let it fail later */ obj.id()
                })
            })
        }

        let (from, to) = (into_tree(&self.repo, from)?, into_tree(&self.repo, to)?);
        let diff = self.repo.diff_tree_to_tree(Some(&from), Some(&to), None)?;
        let mut res = Vec::new();
        diff.print(DiffFormat::Patch,
                   |delta: DiffDelta, hunk: Option<DiffHunk>, diffline: DiffLine| -> bool {
                       if !match delta.status() {
                           Delta::Added | Delta::Modified => true,
                           _ => false,
                       } {
                           return true;
                       }

                       let hunk: DiffHunk = match hunk {
                           Some(h) => h,
                           None => return true,
                       };
                       let content = match str::from_utf8(diffline.content()) {
                           Ok(c) => c,
                           Err(_) => return true,
                       };
                       println!("hunk.new_lines() = {:?}", hunk.new_lines());
                       println!("diffline.new_lineno() = {:?}", diffline.new_lineno());
                       println!("diffline.old_lineno() = {:?}", diffline.old_lineno());
                       println!("diffline.origin() = {:?}", diffline.origin());
                       println!("delta.status() = {:?}", delta.status());
                       println!("diffline.content() = {}", str::from_utf8(diffline.content()).unwrap());
                       println!("diffline.num_lines() = {:?}", diffline.num_lines());
                       if diffline.origin() != '+' {
                           return true
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
