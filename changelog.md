## v7.1.2 (2020-05-28)

* Documentation update

## v7.1.1 (2020-05-23)

* Make new feature from v7.1.0 actually usable

## v7.1.0 (2020-05-23)

* Add `Index::from_path_or_cloned_with_options(…)` to allow cloning from different crates repositories.

## v7.0.1 (2020-03-29)

* disable unused dependency in git2 for lower compile times

## v7.0.0 (2020-03-22)

* update dependencies and upgrade git2 to 0.13.0

## v6.2.0 (2020-03-06)

* Add support for setting the last seen reference directly. Useful in conjunction with `peek_changes(…)`.

## v6.1.0 (2020-03-03)

* Add support for progress messages by adding the `(fetch|peek)_changes_with_options(git2::FetchOptions)` variants.

## v6.0.0 (2020-03-03)

* Update to git2 v0.12.0
  
## v5.1.0 (2020-03-01)

* add `peek_changes()` method, which is like `fetch_changes()`, but doesn't remember which changes it already saw. Use `indx.last_seen_reference().set_target(oid)`
  to get a similar effect as if `fetch_changes()` had been called.
  
## v5.0.2 (2020-02-02)

* speed up diff parsing - skip conversion to utf8

## v5.0.1 (2020-02-02)

* expose the 'git2' crate - useful for error handling

## v5.0.0 (2020-02-01)

* update to libgit 0.11
* provide all information known about the crates, similar to the `crates-index` crate

## v4.0.2 (2019-07-22)

* update dependencies

## v4.0.0 (2018-03-17)

* switch from rustc-serialize to serde

### Breaking Changes

* `CrateVersion::from_crates_diff_json(...)` was removed in favor of `CrateVersion::from_str(...)`
  which is powered by `serde`.

## v3.0.0 (2016-12-30)

* use git2 v0.6 instead of v0.4 to support openssl-sys 0.9.


### v2.0.1 (2016-12-27)

Add a tutorial to the documentation.


## v2.0.0 (2016-12-26)


#### Bug Fixes

* **cargo:**  use git2 version for compat with docs.rs ([0ceebed3](https://github.com/Byron/crates-index-diff-rs/commit/0ceebed3d70c4482b5d09ffa1f9af5fea2bf7cd7))

### v1.0.1 (2016-12-26)


#### Bug Fixes

* **makefile:**  make quick tests quick again ([9aa756ae](https://github.com/Byron/crates-index-diff-rs/commit/9aa756ae534e78fc1c9148a0f6eda27ff07350b5))

#### Features

* **display:**  implementation for changetype ([8ed9a81f](https://github.com/Byron/crates-index-diff-rs/commit/8ed9a81f0a84c43944f29f8407554303d84f7248))



