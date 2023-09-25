use crate::index::{CloneOptions, LAST_SEEN_REFNAME};
use crate::Index;
use std::path::Path;
use std::sync::atomic::AtomicBool;

/// The error returned by various initialization methods.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    PrepareClone(#[from] gix::clone::Error),
    #[error(transparent)]
    Fetch(#[from] gix::clone::fetch::Error),
    #[error(transparent)]
    Open(#[from] gix::open::Error),
}

/// Initialization
impl Index {
    /// Return a new `Index` instance from the given `path`, which should contain a bare clone of the `crates.io` index.
    /// If the directory does not contain the repository or does not exist, it will be cloned from
    /// the official location automatically (with complete history).
    ///
    /// An error will occour if the repository exists and the remote URL does not match the given repository URL.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::atomic::AtomicBool;
    /// use crates_index_diff::{Index, index, git};
    ///
    /// # let path = tempdir::TempDir::new("index").unwrap();
    /// // Note that credentials are automatically picked up from the standard git configuration.
    /// let mut options = index::CloneOptions {
    ///   url: "https://github.com/rust-lang/staging.crates.io-index".into(),
    /// };
    ///
    ///
    /// let index = Index::from_path_or_cloned_with_options(path, gix::progress::Discard, &AtomicBool::default(), options)?;
    /// # Ok::<(), crates_index_diff::index::init::Error>(())
    /// ```
    pub fn from_path_or_cloned_with_options<P>(
        path: impl AsRef<Path>,
        progress: P,
        should_interrupt: &AtomicBool,
        CloneOptions { url }: CloneOptions,
    ) -> Result<Index, Error>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
    {
        let path = path.as_ref();
        let mut repo = match gix::open(path) {
            Ok(repo) => repo,
            Err(gix::open::Error::NotARepository { .. }) => {
                let (repo, _out) =
                    gix::prepare_clone_bare(url, path)?.fetch_only(progress, should_interrupt)?;
                repo
            }
            Err(err) => return Err(err.into()),
        };

        repo.object_cache_size_if_unset(4 * 1024 * 1024);
        let remote_name = repo
            .remote_names()
            .into_iter()
            .next()
            .map(ToOwned::to_owned);
        Ok(Index {
            repo,
            remote_name,
            branch_name: "master",
            seen_ref_name: LAST_SEEN_REFNAME,
        })
    }

    /// Return a new `Index` instance from the given `path`, which should contain a bare or non-bare
    /// clone of the `crates.io` index.
    /// If the directory does not contain the repository or does not exist, it will be cloned from
    /// the official location automatically (with complete history).
    pub fn from_path_or_cloned(path: impl AsRef<Path>) -> Result<Index, Error> {
        Index::from_path_or_cloned_with_options(
            path,
            gix::progress::Discard,
            &AtomicBool::default(),
            CloneOptions::default(),
        )
    }
}
