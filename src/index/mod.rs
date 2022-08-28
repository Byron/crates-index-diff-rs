use crate::Index;
use git_repository as git;
use std::str;

static INDEX_GIT_URL: &str = "https://github.com/rust-lang/crates.io-index";
static LAST_SEEN_REFNAME: &str = "refs/heads/crates-index-diff_last-seen";

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
