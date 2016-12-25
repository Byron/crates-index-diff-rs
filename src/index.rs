use std::path::Path;

use git2::Repository;
use git2::build::RepoBuilder;
use git2::{ErrorClass, Error as GitError};

static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";

pub struct Index {
    repo: Repository,
}

impl Index {
    pub fn at_path<P>(path: P) -> Result<Index, GitError>
        where P: AsRef<Path>
    {
        let repo = Repository::open(path.as_ref()).or_else(|err: GitError| if err.class() == ErrorClass::Repository {
            RepoBuilder::new().bare(true).clone(INDEX_GIT_URL, path.as_ref())
        } else {
            Err(err)
        })?;

        Ok(Index { repo: repo })
    }
}
