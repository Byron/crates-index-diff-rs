use crate::Index;
use std::str;

static INDEX_GIT_URL: &str = "https://github.com/rust-lang/crates.io-index";
static LAST_SEEN_REFNAME: &str = "refs/heads/crates-index-diff_last-seen";

/// Declarative macro to generate `impl From<Src> for Error` where the source
/// error value is boxed into the given `Error` enum variant.
///
/// Usage:
/// impl_from_boxed!(gix::object::find::existing::Error => ErrorEnum::FindObject);
/// impl_from_boxed!(gix::remote::find::existing::Error => ErrorEnum::FindRemote);
///
/// See also:
/// * https://rust-lang.github.io/rust-clippy/stable/index.html#/result_large_err
/// * https://github.com/dtolnay/thiserror/pull/419
/// * https://github.com/dtolnay/thiserror/pull/418
/// * https://github.com/dtolnay/thiserror/issues/302
macro_rules! impl_from_boxed {
    ($src:path => $err:ident::$variant:ident) => {
        impl From<$src> for $err {
            fn from(value: $src) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

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
    pub fn repository(&self) -> &gix::Repository {
        &self.repo
    }

    /// Return the crates.io repository, mutably.
    pub fn repository_mut(&mut self) -> &mut gix::Repository {
        &mut self.repo
    }

    /// Return the reference pointing to the state we have seen after calling `fetch_changes()`.
    pub fn last_seen_reference(
        &self,
    ) -> Result<gix::Reference<'_>, gix::reference::find::existing::Error> {
        self.repo.find_reference(self.seen_ref_name)
    }
}

/// Main index diff functionality
pub mod diff;
/// initial index repo loading & cloning
pub mod init;
