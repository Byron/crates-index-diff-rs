<a name="v5.0.2"></a>
## v5.0.1 (2020-02-02)

* expose the 'git2' crate - useful for error handling

<a name="v5.0.2"></a>
## v5.0.0 (2020-02-01)

* update to libgit 0.11
* provide all information known about the crates, similar to the `crates-index` crate

<a name="v4.0.2"></a>
## v4.0.2 (2019-07-22)

* update dependencies

<a name="v4.0.0"></a>
## v4.0.0 (2018-03-17)

* switch from rustc-serialize to serde

### Breaking Changes

* `CrateVersion::from_crates_diff_json(...)` was removed in favor of `CrateVersion::from_str(...)`
  which is powered by `serde`.

<a name="v3.0.0"></a>
## v3.0.0 (2016-12-30)

* use git2 v0.6 instead of v0.4 to support openssl-sys 0.9.


<a name="v2.0.1"></a>
### v2.0.1 (2016-12-27)

Add a tutorial to the documentation.


<a name="v2.0.0"></a>
## v2.0.0 (2016-12-26)


#### Bug Fixes

* **cargo:**  use git2 version for compat with docs.rs ([0ceebed3](https://github.com/Byron/crates-index-diff-rs/commit/0ceebed3d70c4482b5d09ffa1f9af5fea2bf7cd7))



<a name="v1.0.1"></a>
### v1.0.1 (2016-12-26)


#### Bug Fixes

* **makefile:**  make quick tests quick again ([9aa756ae](https://github.com/Byron/crates-index-diff-rs/commit/9aa756ae534e78fc1c9148a0f6eda27ff07350b5))

#### Features

* **display:**  implementation for changetype ([8ed9a81f](https://github.com/Byron/crates-index-diff-rs/commit/8ed9a81f0a84c43944f29f8407554303d84f7248))



