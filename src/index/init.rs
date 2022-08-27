use crate::index::{CloneOptions, LAST_SEEN_REFNAME};
use crate::Index;
use std::path::Path;

/// Initialization
impl Index {
    /// Return a new `Index` instance from the given `path`, which should contain a bare or non-bare
    /// clone of the `crates.io` index.
    /// If the directory does not contain the repository or does not exist, it will be cloned from
    /// the official location automatically (with complete history).
    ///
    /// An error will occour if the repository exists and the remote URL does not match the given repository URL.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crates_index_diff::{Index, index};
    ///
    /// # let path = tempdir::TempDir::new("index").unwrap();
    /// let mut options = index::CloneOptions {
    ///   repository_url: "https://github.com/rust-lang/staging.crates.io-index".into(),
    ///   ..Default::default()
    /// };
    ///
    ///
    /// let index = Index::from_path_or_cloned_with_options(path, options)?;
    /// # Ok::<(), git2::Error>(())
    /// ```
    /// Or to access a private repository, use fetch options.
    ///
    /// ```no_run
    /// use crates_index_diff::{index, Index};
    /// let fo = {
    ///     let mut fo = git2::FetchOptions::new();
    ///     fo.remote_callbacks({
    ///         let mut callbacks = git2::RemoteCallbacks::new();
    ///         callbacks.credentials(|_url, username_from_url, _allowed_types| {
    ///             git2::Cred::ssh_key_from_memory(
    ///                 username_from_url.unwrap(),
    ///                 None,
    ///                 &std::env::var("PRIVATE_KEY").unwrap(),
    ///                 None,
    ///             )
    ///         });
    ///         callbacks
    ///     });
    ///     fo
    /// };
    /// Index::from_path_or_cloned_with_options(
    ///     "index",
    ///     index::CloneOptions {
    ///         repository_url: "git@github.com:private-index/goes-here.git".into(),
    ///         fetch_options: Some(fo),
    ///     },
    /// ).unwrap();
    /// ```
    pub fn from_path_or_cloned_with_options(
        path: impl AsRef<Path>,
        CloneOptions {
            repository_url,
            fetch_options,
        }: CloneOptions<'_>,
    ) -> Result<Index, git2::Error> {
        let mut repo_did_exist = true;
        let repo = git2::Repository::open(path.as_ref()).or_else(|err| {
            if err.class() == git2::ErrorClass::Repository {
                repo_did_exist = false;
                let mut builder = git2::build::RepoBuilder::new();
                if let Some(fo) = fetch_options {
                    builder.fetch_options(fo);
                }
                builder.bare(true).clone(&repository_url, path.as_ref())
            } else {
                Err(err)
            }
        })?;

        Ok(Index {
            repo,
            seen_ref_name: LAST_SEEN_REFNAME,
        })
    }

    /// Return a new `Index` instance from the given `path`, which should contain a bare or non-bare
    /// clone of the `crates.io` index.
    /// If the directory does not contain the repository or does not exist, it will be cloned from
    /// the official location automatically (with complete history).
    pub fn from_path_or_cloned(path: impl AsRef<Path>) -> Result<Index, git2::Error> {
        Index::from_path_or_cloned_with_options(path, CloneOptions::default())
    }
}
