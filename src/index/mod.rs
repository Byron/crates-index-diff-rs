use crate::Index;
use git_repository as git;
use std::str;

static INDEX_GIT_URL: &str = "https://github.com/rust-lang/crates.io-index";
static LAST_SEEN_REFNAME: &str = "refs/heads/crates-index-diff_last-seen";

/// Options for cloning the crates-io index.
pub struct CloneOptions {
    /// The url to clone the crates-index repository from.
    pub url: String,
}

impl Default for CloneOptions {
    fn default() -> Self {
        CloneOptions {
            url: INDEX_GIT_URL.into(),
        }
    }
}

/// Access
impl Index {
    /// Return the crates.io repository.
    pub fn repository(&self) -> &git::Repository {
        &self.repo
    }

    /// Return the crates.io repository, mutably.
    pub fn repository_mut(&mut self) -> &mut git::Repository {
        &mut self.repo
    }

    /// Return the reference pointing to the state we have seen after calling `fetch_changes()`.
    pub fn last_seen_reference(
        &self,
    ) -> Result<git::Reference<'_>, git::reference::find::existing::Error> {
        self.repo.find_reference(self.seen_ref_name)
    }
}

///
pub mod diff;
///
pub mod init;
