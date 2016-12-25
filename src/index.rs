use std::path::Path;

use git2::build::RepoBuilder;
use git2::{Tree, Repository, Oid, ErrorClass, Error as GitError};

static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";

pub struct Index {
    repo: Repository,
}

pub enum ChangeType {
    Added
}

pub struct Crate {
    pub name: String,
    pub state: ChangeType
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

    pub fn traverse_changes<S1, S2>(&self, from: S1, to: S2) -> Result<Vec<Crate>, GitError>
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        fn into_tree<S: AsRef<str>>(repo: &Repository, rev: S) -> Result<Tree, GitError> {
            repo.revparse_single(rev.as_ref()).and_then(|obj| repo.find_tree(obj.id()))
        }

        let (from, to) = (into_tree(&self.repo, from)?, into_tree(&self.repo, to)?);
        Ok(Vec::new())
    }
}
