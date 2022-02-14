# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 8.0.1 (2022-02-14)

 - Only download the master branch on clone, not all branches, to greatly reduce the initial download size from nearly 800MB to just about 100MB.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 197 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare changelog ([`76ae38d`](https://github.com/Byron/crates-index-diff-rs/commit/76ae38d8bccc3518c11bb9f5154b8fd8a993e13f))
    - Add some more debugging info to failing test ([`b111e03`](https://github.com/Byron/crates-index-diff-rs/commit/b111e037619a9e36f5227957b46eaaeece8321f4))
    - Only fetch the master crates.io branch, not all branches ([`79cf4ca`](https://github.com/Byron/crates-index-diff-rs/commit/79cf4cab3c444b3c53c43050fd222db984e4c717))
</details>

## v8.0.0 (2021-07-31)

* Add fetch-options to `CloneOptions` to allow private crate index repositories

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 429 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#11](https://github.com/Byron/crates-index-diff-rs/issues/11)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#11](https://github.com/Byron/crates-index-diff-rs/issues/11)**
    - Add example for fetch options. ([`eaef987`](https://github.com/Byron/crates-index-diff-rs/commit/eaef987e661070aa078db6a47f535343fc1015a4))
 * **Uncategorized**
    - (cargo-release) version 8.0.0 ([`d6f6d46`](https://github.com/Byron/crates-index-diff-rs/commit/d6f6d46aa00256e57eccfb73d626602c926f2dc9))
    - remove non-exhaustive attribute… ([`fbf93be`](https://github.com/Byron/crates-index-diff-rs/commit/fbf93beb4f6d9a496adaaece8bdae6f67c2149e9))
    - update changelog ([`79a2ea5`](https://github.com/Byron/crates-index-diff-rs/commit/79a2ea5a472a4bf7cfd06662f75eca0688fb83ac))
    - Allow future additions of more options without API breakage ([`919d9da`](https://github.com/Byron/crates-index-diff-rs/commit/919d9da05098472264b0faf5adc72cc3c36bf6fd))
    - Pass FetchOptions for cloning ([`091ef7d`](https://github.com/Byron/crates-index-diff-rs/commit/091ef7dd39a8cbd2730785504ad6817e3d3141e9))
</details>

## v7.1.2 (2020-05-28)

* Documentation update

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 4 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Inform about resource usage in the docs ([`386dcc4`](https://github.com/Byron/crates-index-diff-rs/commit/386dcc4f1cd5118e71a0664c623e8e2a6c77e0ff))
    - optimize includes using 'cargo diet' ([`0989e0f`](https://github.com/Byron/crates-index-diff-rs/commit/0989e0f59da7ce82cfb92198ac8bf713e4e209d2))
</details>

## v7.1.1 (2020-05-23)

* Make new feature from v7.1.0 actually usable

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump patch level ([`fceb154`](https://github.com/Byron/crates-index-diff-rs/commit/fceb154bdfb99eb98580906161fce452854ed78a))
    - Avoid cloning the crates.io index as part of the doctests; convenience ([`dd869e2`](https://github.com/Byron/crates-index-diff-rs/commit/dd869e2bf29cb85901ea3fc6bab97296dd336a09))
    - Allow changing CloneOptions::repository_url from outside the crate ([`2f10281`](https://github.com/Byron/crates-index-diff-rs/commit/2f10281647279cb4586329e5c9b4eaeb61da53ef))
</details>

## v7.1.0 (2020-05-23)

* Add `Index::from_path_or_cloned_with_options(…)` to allow cloning from different crates repositories.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 43 calendar days.
 - 54 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version ([`0f20006`](https://github.com/Byron/crates-index-diff-rs/commit/0f200065d1c829a526b3376035e55ec9c3b2b57a))
    - add Index::from_path_or_cloned_with_options(…) to support different crates repositories ([`f9b991c`](https://github.com/Byron/crates-index-diff-rs/commit/f9b991c8f91a378ad14d04fced67ec9cda8c3bf7))
    - bye bye travis, we had a good time. ([`d913fc1`](https://github.com/Byron/crates-index-diff-rs/commit/d913fc1bd7bc7dbc19a012acaaf7d4757ed0f5e7))
    - Add github actions ([`204dcd7`](https://github.com/Byron/crates-index-diff-rs/commit/204dcd75ce382a5daab3931c7e9529ff1b2772bf))
    - Update README to inform about the collapsed crates.io history, and that we deal with it ([`5966b49`](https://github.com/Byron/crates-index-diff-rs/commit/5966b494415421700bb48548f728e8d560f12e2e))
</details>

## v7.0.1 (2020-03-29)

* disable unused dependency in git2 for lower compile times

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 7 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump patch ([`bf8f77d`](https://github.com/Byron/crates-index-diff-rs/commit/bf8f77dd7dac7a3e5efe6bfa1404f20acf0c885f))
    - Disable unused ssh git feature ([`3265093`](https://github.com/Byron/crates-index-diff-rs/commit/326509334e04658be4ed86902571701c29eac1a2))
</details>

## v7.0.0 (2020-03-22)

* update dependencies and upgrade git2 to 0.13.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 16 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update dependencies ([`ed5c1ac`](https://github.com/Byron/crates-index-diff-rs/commit/ed5c1ac827b48b400bc7982adf1e254fc5861f33))
</details>

## v6.2.0 (2020-03-06)

* Add support for setting the last seen reference directly. Useful in conjunction with `peek_changes(…)`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - refactor for more flexibility; bump version ([`eb244b7`](https://github.com/Byron/crates-index-diff-rs/commit/eb244b7e2ba4a860b6979d7eb8fafdf7cdcf5517))
</details>

## v6.1.0 (2020-03-02)

* Add support for progress messages by adding the `(fetch|peek)_changes_with_options(git2::FetchOptions)` variants.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Support for progress messages ([`0167119`](https://github.com/Byron/crates-index-diff-rs/commit/0167119f67aa76ced8a45e708f6bf6b4e4345c37))
</details>

## v6.0.0 (2020-03-02)

* Update to git2 v0.12.0
  

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Upgrade to git2 v0.12.0 ([`6456fa2`](https://github.com/Byron/crates-index-diff-rs/commit/6456fa2710341e29aa8a0adb787274628688f99d))
</details>

## v5.1.0 (2020-03-01)

* add `peek_changes()` method, which is like `fetch_changes()`, but doesn't remember which changes it already saw. Use `indx.last_seen_reference().set_target(oid)`
  to get a similar effect as if `fetch_changes()` had been called.
  

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 10 calendar days.
 - 10 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Allow to peek changes ([`99c33e6`](https://github.com/Byron/crates-index-diff-rs/commit/99c33e6936dadd7b3ab0af6709791a8cbdda2071))
    - Don't borrow from git… Revert "Use Cow everywhere in crate version" ([`8c2f439`](https://github.com/Byron/crates-index-diff-rs/commit/8c2f4397d7d3e637b9b3c7d9a19b5b5502ce9082))
    - Use Cow everywhere in crate version ([`8d10090`](https://github.com/Byron/crates-index-diff-rs/commit/8d1009058150861c5682709162da47daa8d4b192))
    - (cargo-release) start next development iteration 5.0.6-alpha.0 ([`5ccb705`](https://github.com/Byron/crates-index-diff-rs/commit/5ccb7058c85960a8ae13334e8f0bfc6bdf7dad66))
</details>

## v5.0.5 (2020-02-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 13 calendar days.
 - 13 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Don't skip serializing None fields… ([`11f4aa0`](https://github.com/Byron/crates-index-diff-rs/commit/11f4aa02bcaedc290f1964a909fbf9e8fec06eb6))
    - (cargo-release) start next development iteration 5.0.5-alpha.0 ([`992a71d`](https://github.com/Byron/crates-index-diff-rs/commit/992a71dbb8ddabc4da8f67d005def5c5d10c55d5))
</details>

## v5.0.4 (2020-02-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Implement `Default` for ChangeKind ([`2234f7b`](https://github.com/Byron/crates-index-diff-rs/commit/2234f7b4c846ea8bee7241b43b1613d3544e8e5d))
    - (cargo-release) start next development iteration 5.0.4-alpha.0 ([`fac9ee1`](https://github.com/Byron/crates-index-diff-rs/commit/fac9ee1a3000c133b4c565a3b382ca62aa778463))
</details>

## v5.0.3 (2020-02-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add missing clone trait ([`5f6e3a2`](https://github.com/Byron/crates-index-diff-rs/commit/5f6e3a25ad92378cc2c472239784f7ce6b501ad4))
    - (cargo-release) start next development iteration 5.0.3-alpha.0 ([`b8c8412`](https://github.com/Byron/crates-index-diff-rs/commit/b8c8412fd68beb3fb535ef7e93f50d8d4786fdac))
</details>

## v5.0.2 (2020-02-02)

* speed up diff parsing - skip conversion to utf8

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Improved diff parsing - should be a bit faster ([`b819af0`](https://github.com/Byron/crates-index-diff-rs/commit/b819af0b23ae2abbacae1ad3800290b9bb658a2f))
    - (cargo-release) start next development iteration 5.0.2-alpha.0 ([`ab59e70`](https://github.com/Byron/crates-index-diff-rs/commit/ab59e70d0533492946275ec4c0e84ba54d4fe87b))
</details>

## v5.0.1 (2020-02-02)

* expose the 'git2' crate - useful for error handling

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Expose git2; bump patch level ([`df46581`](https://github.com/Byron/crates-index-diff-rs/commit/df465810d95362d224c75d9b277df6160ab715aa))
    - Add README to crates.io ([`1ca58b2`](https://github.com/Byron/crates-index-diff-rs/commit/1ca58b25766c6280d76669ccb5988e838f80c98e))
    - Add README to crates.io ([`5f78bde`](https://github.com/Byron/crates-index-diff-rs/commit/5f78bde1517852b91b36fe962f40a90925d24d48))
    - Update README ([`b2bb821`](https://github.com/Byron/crates-index-diff-rs/commit/b2bb821e7e6eb2f01e68a5501cb01fefa549b30c))
</details>

## v5.0.0 (2020-02-01)

* update to libgit 0.11
* provide all information known about the crates, similar to the `crates-index` crate

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 163 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix unit tests ([`70b3d01`](https://github.com/Byron/crates-index-diff-rs/commit/70b3d0132dd9e6ddbefa63049f49a0a303f4fe26))
    - modernize code a little ([`00c0cb7`](https://github.com/Byron/crates-index-diff-rs/commit/00c0cb70cab3b348f071adaa8ff49d6c41ecd1b0))
    - upgrade git; add all crate-fields ([`108b15e`](https://github.com/Byron/crates-index-diff-rs/commit/108b15e9d6a49bbdcb7d15a4aea04a5c4cde40ad))
</details>

## v4.0.2 (2019-08-22)

* update dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 47 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version after dependency update ([`f49bbda`](https://github.com/Byron/crates-index-diff-rs/commit/f49bbda57187321178621b62187cb18ad5f966eb))
    - Update git2 crate ([`e1152c2`](https://github.com/Byron/crates-index-diff-rs/commit/e1152c2a4d8850ba3fb72aac0a37a6b51ae85fbe))
</details>

## v4.0.1 (2019-07-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 474 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump patch level: update git2 dependency ([`debd65b`](https://github.com/Byron/crates-index-diff-rs/commit/debd65b2fee87d2b93a4701abe4871ef4b89c8b5))
    - bump `git2` version ([`724f822`](https://github.com/Byron/crates-index-diff-rs/commit/724f8228ed0ab17d01c5dbec0a88630413094ce7))
</details>

## v4.0.0 (2018-03-17)

* switch from rustc-serialize to serde

### Breaking Changes

* `CrateVersion::from_crates_diff_json(...)` was removed in favor of `CrateVersion::from_str(...)`
  which is powered by `serde`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 4 calendar days.
 - 441 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version after switching to serde ([`2498da9`](https://github.com/Byron/crates-index-diff-rs/commit/2498da9cb5583ed33116e4bbea3a4da144c178c7))
    - Change refspec back from 'master' to what works in tests ([`20d2c0e`](https://github.com/Byron/crates-index-diff-rs/commit/20d2c0e221915f4c053adeba2de28a23d9a8b035))
    - Upgrade git2 version ([`fde4a47`](https://github.com/Byron/crates-index-diff-rs/commit/fde4a4785556dcc8a204958ccf794217641b92f8))
    - Add serde test ([`c0c3172`](https://github.com/Byron/crates-index-diff-rs/commit/c0c317294df78d584176cf1d1948ce82ee3a7a00))
    - Use serde instead of rustc-serialize ([`a310303`](https://github.com/Byron/crates-index-diff-rs/commit/a31030349bc2b38aea1807ae3d07d129ca724b0f))
    - Fix failing test ([`fd37e49`](https://github.com/Byron/crates-index-diff-rs/commit/fd37e49f4d34dc1cc183bd791b4f66038ffa8fd5))
</details>

## v3.0.0 (2016-12-30)

<csr-id-38d9163ed007f2d925201f9a4cb2b4e3d0758dab/>

* use git2 v0.6 instead of v0.4 to support openssl-sys 0.9.


### Chore

 - <csr-id-38d9163ed007f2d925201f9a4cb2b4e3d0758dab/> use latest version of git2
   That way, crates-io-cli can used the latest crates.
   Version two is used by doc.rs and must remain as is.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 3 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - use latest version of git2 ([`38d9163`](https://github.com/Byron/crates-index-diff-rs/commit/38d9163ed007f2d925201f9a4cb2b4e3d0758dab))
</details>

## v2.0.1 (2016-12-27)

Add a tutorial to the documentation.

### Documentation

 - <csr-id-2d0c8163621f650f17a06c82f70d1bbd147a582e/> add reference to usage example.
   We are using the crates-cli as an example, even though
   it certainly is not the cleanest one possible.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add reference to usage example. ([`2d0c816`](https://github.com/Byron/crates-index-diff-rs/commit/2d0c8163621f650f17a06c82f70d1bbd147a582e))
</details>

## v2.0.0 (2016-12-26)

<csr-id-bd89e7267b23d8a0bd801679d1ef74d12c84ba09/>

### Chore

 - <csr-id-bd89e7267b23d8a0bd801679d1ef74d12c84ba09/> update to v2.0.0

### Bug Fixes

* **cargo:**  use git2 version for compat with docs.rs ([0ceebed3](https://github.com/Byron/crates-index-diff-rs/commit/0ceebed3d70c4482b5d09ffa1f9af5fea2bf7cd7))
 - <csr-id-0ceebed3d70c4482b5d09ffa1f9af5fea2bf7cd7/> use git2 version for compat with docs.rs
   Otherwise docs.rs cannot use use.
   This is due to the issue described here:

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update to v2.0.0 ([`bd89e72`](https://github.com/Byron/crates-index-diff-rs/commit/bd89e7267b23d8a0bd801679d1ef74d12c84ba09))
    - use git2 version for compat with docs.rs ([`0ceebed`](https://github.com/Byron/crates-index-diff-rs/commit/0ceebed3d70c4482b5d09ffa1f9af5fea2bf7cd7))
</details>

## v1.0.1 (2016-12-26)

<csr-id-de4a284687fb476dd70bed3a4eb7e1737aff57ea/>

### Bug Fixes

* **makefile:**  make quick tests quick again ([9aa756ae](https://github.com/Byron/crates-index-diff-rs/commit/9aa756ae534e78fc1c9148a0f6eda27ff07350b5))
 - <csr-id-9aa756ae534e78fc1c9148a0f6eda27ff07350b5/> make quick tests quick again
* **display:**  implementation for changetype ([8ed9a81f](https://github.com/Byron/crates-index-diff-rs/commit/8ed9a81f0a84c43944f29f8407554303d84f7248))

### New Features

 - <csr-id-8ed9a81f0a84c43944f29f8407554303d84f7248/> implementation for changetype
 - <csr-id-de4a284687fb476dd70bed3a4eb7e1737aff57ea/> v1.0.1

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 4 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v1.0.1 ([`de4a284`](https://github.com/Byron/crates-index-diff-rs/commit/de4a284687fb476dd70bed3a4eb7e1737aff57ea))
    - implementation for changetype ([`8ed9a81`](https://github.com/Byron/crates-index-diff-rs/commit/8ed9a81f0a84c43944f29f8407554303d84f7248))
    - make quick tests quick again ([`9aa756a`](https://github.com/Byron/crates-index-diff-rs/commit/9aa756ae534e78fc1c9148a0f6eda27ff07350b5))
    - crates.io badge ([`304dfaf`](https://github.com/Byron/crates-index-diff-rs/commit/304dfafe95b23703f3b6d11230b487304d5d6bd0))
</details>

<csr-unknown>
FeaturesChore<csr-unknown/>

## v1.0.0 (2016-12-26)

<csr-id-381e7cc1e59a3695f3f07061467ade159822dbcb/>
<csr-id-a4e6e7efef4bf7d6863eeade1b974a350d08f4a6/>
<csr-id-787bdacfa2c1191e121d4d5a6c8e3a09c19bf684/>
<csr-id-11a006bc230d098ad8ee282069b2544c8187d14a/>
<csr-id-607d747c9306ad0921a6d4166ecf151bb9a39479/>
<csr-id-3ce91e89b520b6497eceb961065e8898c5aba883/>
<csr-id-f1568c02652781163055bcbc18bc4af0e6914cd2/>
<csr-id-601dc2d7ca9bd87f440455e1eb3698c6e2f0227d/>
<csr-id-ef5349677d53ab2d31c921b853766a8aa21f59fa/>
<csr-id-97a5b819df11a454f5154ae3edd5e4a03192c0e1/>
<csr-id-81c95f2a9014ace72e74da64ddb4840197d7bbec/>
<csr-id-988b66860a96411f9c263c582159066970ef9265/>
<csr-id-caa58790f61ec4e450dadbf737a0bc3224f8c0b4/>
<csr-id-b0dd2770a979b278e8be4432418367c6d9620c67/>
<csr-id-eee513476d300c294272e0c23348cdbc1009f008/>
<csr-id-42e68031d586109efe7d9567279af4d4bf7ac0be/>
<csr-id-d06c4a0bb526eafb13a61951cd34c24febd10797/>
<csr-id-97b417ad71b3070e3663d1ce6d998f24a0bf1365/>
<csr-id-fdf281ee3b301ab370fe1d3ea6aeb1dd5e0536d1/>
<csr-id-1c0b2c0a3723afd587779880337f5a5516c063f1/>

### Chore

 - <csr-id-381e7cc1e59a3695f3f07061467ade159822dbcb/> v1.0.0
 - <csr-id-a4e6e7efef4bf7d6863eeade1b974a350d08f4a6/> be compatible with docs.rs
   See
 - <csr-id-787bdacfa2c1191e121d4d5a6c8e3a09c19bf684/> fix keywords
 - <csr-id-11a006bc230d098ad8ee282069b2544c8187d14a/> see why makefile is not working
 - <csr-id-607d747c9306ad0921a6d4166ecf151bb9a39479/> found an illegal-instruction error
   Just run cargo test
 - <csr-id-3ce91e89b520b6497eceb961065e8898c5aba883/> intermediate crate info from hunk
   However, it's not yet working
 - <csr-id-f1568c02652781163055bcbc18bc4af0e6914cd2/> first commit

### New Features

 - <csr-id-accb62d00618204e76659ecb4d31e8a04291bdc5/> using openssl file as basis

### Bug Fixes

 - <csr-id-cdfc689aacd68dd53ecef19187d1a02de84a8ab7/> give up on osx
   Openssl can't be linked due to missing symbols, no matter what.
   
   Maybe ... the version is too new?
   Odd that it works locally though.
 - <csr-id-3a5eb46249b6e319944efd53e5003024125e286c/> see if openssl somehow works after all
 - <csr-id-36b79decb47d541172fffcbf6b43c60ab5bdd8d5/> openssl setup from openssl-rs itself
 - <csr-id-120d752b658850df962d65e50fe054c7782b1b04/> try the msvc 32bit version
 - <csr-id-42a9800b03894dd3dc58e3ceefa1c9fecc7b589d/> msys2 seems to be the problem here
 - <csr-id-0a738f3513c1b66f12b99c4260728dcbb3c41e68/> try to enforce linking latest openssl
 - <csr-id-dda7c80ae297084a364b91b36619995c130ee24c/> try disabling the cache
   Maybe that way, the 64 bit builds will work
 - <csr-id-714d7cf2ce6de29a5021cf5c2c85e5b14631e203/> enforce more recent openssl lib
   Currently some symbols are missing or are named differently.
   Let's also see if it links it to the correct spot in /usr/local/opt/openssl
 - <csr-id-ae26330ee78be142f63c41d303a8ac604083880e/> only 1.1.0c seems to be working
   See
 - <csr-id-959fde03288238dc74ec82ebb2a949e834481ed2/> remove 32 bit targets due to failure
   The previous version triggered this error:
   Error parsing appveyor.yml: Value can be either string or "secure" string. (Line: 5, Column: 7)
 - <csr-id-f4ea940e9e8f6adc6a82a45adebecf01c26b7851/> attempt to set OPENSSL env vars
 - <csr-id-05bffd68e3411ca4327c7a905c9a4e562c9c2eb3/> try to allow 32 bit failures
   Could also remove them, but ideally it remains visible to maybe
   one day work.
 - <csr-id-b11f6df25e6a12e4835f32b739b7c7fd890d6c23/> try without index-bare cache
   For now ... I have not seen it being cloned at all.
   Possibly the cache provides a none-empty directory ?
 - <csr-id-48609062e30407ea29afa5ddccea78e7c94f20bd/> custom code to get cmake onto osx
 - <csr-id-a052f41cd198b7342b14aba186a955067a9a7241/> let's hope it will actually run now
 - <csr-id-b784f1cb3655a2ebef7845ed62cd5c52ab31d46e/> set envvars propery
 - <csr-id-24c1c81d7e1287ed83308f4f8ad795dfca4de834/> remove debug prints
 - <csr-id-9ef08fa5d1517883cb006fda522d020767f8d081/> remove error handling in makefile
   This isn't a good thing to do for all of those without cargo.
   However, let's see if it fixes the issue first.
 - <csr-id-42e1290ea75ddf57948b6f6acd23c2c17544db06/> to work with GNU make
 - <csr-id-f89651be89638e8210b74dc8843d5985d28e842b/> finally the correct refspec
   Fair enough, need trees here after all!
 - <csr-id-d0d43071a3fdcd795a2460b71363545dbf5a2a89/> cmake binary installation
 - <csr-id-f9f67efd7ca12c34d4bbfa171b8604f206d686b4/> use different cmake source
 - <csr-id-a97813620f5990f4d62337e7a6a8c3b3aadd05ef/> cmake upgrade via before_script
 - <csr-id-77bbd20132b7f42dd79b55f86c000f21618d4c39/> see if a custom source will get latest cmake
 - <csr-id-2d0d5d94193d5a65cdac700d811e9d75e849a7c9/> enforce latest cmake

### Refactor

 - <csr-id-601dc2d7ca9bd87f440455e1eb3698c6e2f0227d/> don't enforce branches
   Previously valid ref-names would be interpreted as branches, which
   limits their use despite the name indicating otherwise.
   
   Now every ref path is supported.
 - <csr-id-ef5349677d53ab2d31c921b853766a8aa21f59fa/> use quick-error
   That way, the crate-error type is an actual error with
   all the relevant implementations.
   
   It's impossible to work without it, I think :).
 - <csr-id-97a5b819df11a454f5154ae3edd5e4a03192c0e1/> move crate into own module
 - <csr-id-81c95f2a9014ace72e74da64ddb4840197d7bbec/> better diff.print implementation
 - <csr-id-988b66860a96411f9c263c582159066970ef9265/> simplify fetch-changes error handling
 - <csr-id-caa58790f61ec4e450dadbf737a0bc3224f8c0b4/> much nicer Crate from json
   It's good to know that we can chain everything, but that
   should only be done if there is an actual dependency of the
   respective values.
   
   Otherwise, just abort early.
 - <csr-id-b0dd2770a979b278e8be4432418367c6d9620c67/> better error handling
 - <csr-id-eee513476d300c294272e0c23348cdbc1009f008/> allow direct usage of objects as well

### Test

 - <csr-id-42e68031d586109efe7d9567279af4d4bf7ac0be/> remove duplicate
   all crates are also iterated in the
   'changed_since_last_fetch' test.
 - <csr-id-d06c4a0bb526eafb13a61951cd34c24febd10797/> fetch_changes()
   Just the frame and all testing I could think of
 - <csr-id-97b417ad71b3070e3663d1ce6d998f24a0bf1365/> assure we can handle the entire index
 - <csr-id-fdf281ee3b301ab370fe1d3ea6aeb1dd5e0536d1/> first clone on demand
   Would be nice to be able to specify --depth as well.
 - <csr-id-1c0b2c0a3723afd587779880337f5a5516c063f1/> first failing test

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 61 commits contributed to the release over the course of 1 calendar day.
 - 61 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v1.0.0 ([`381e7cc`](https://github.com/Byron/crates-index-diff-rs/commit/381e7cc1e59a3695f3f07061467ade159822dbcb))
    - docs for all remaining methods. ([`706636b`](https://github.com/Byron/crates-index-diff-rs/commit/706636b5198595ff8573505350f49aad241edfc6))
    - give up on osx ([`cdfc689`](https://github.com/Byron/crates-index-diff-rs/commit/cdfc689aacd68dd53ecef19187d1a02de84a8ab7))
    - see if openssl somehow works after all ([`3a5eb46`](https://github.com/Byron/crates-index-diff-rs/commit/3a5eb46249b6e319944efd53e5003024125e286c))
    - documentation for crateversion ([`91bf44d`](https://github.com/Byron/crates-index-diff-rs/commit/91bf44d4f3c4454316f32489ba30cd250422065d))
    - openssl setup from openssl-rs itself ([`36b79de`](https://github.com/Byron/crates-index-diff-rs/commit/36b79decb47d541172fffcbf6b43c60ab5bdd8d5))
    - try the msvc 32bit version ([`120d752`](https://github.com/Byron/crates-index-diff-rs/commit/120d752b658850df962d65e50fe054c7782b1b04))
    - msys2 seems to be the problem here ([`42a9800`](https://github.com/Byron/crates-index-diff-rs/commit/42a9800b03894dd3dc58e3ceefa1c9fecc7b589d))
    - try to enforce linking latest openssl ([`0a738f3`](https://github.com/Byron/crates-index-diff-rs/commit/0a738f3513c1b66f12b99c4260728dcbb3c41e68))
    - try disabling the cache ([`dda7c80`](https://github.com/Byron/crates-index-diff-rs/commit/dda7c80ae297084a364b91b36619995c130ee24c))
    - enforce more recent openssl lib ([`714d7cf`](https://github.com/Byron/crates-index-diff-rs/commit/714d7cf2ce6de29a5021cf5c2c85e5b14631e203))
    - only 1.1.0c seems to be working ([`ae26330`](https://github.com/Byron/crates-index-diff-rs/commit/ae26330ee78be142f63c41d303a8ac604083880e))
    - remove 32 bit targets due to failure ([`959fde0`](https://github.com/Byron/crates-index-diff-rs/commit/959fde03288238dc74ec82ebb2a949e834481ed2))
    - attempt to set OPENSSL env vars ([`f4ea940`](https://github.com/Byron/crates-index-diff-rs/commit/f4ea940e9e8f6adc6a82a45adebecf01c26b7851))
    - try to allow 32 bit failures ([`05bffd6`](https://github.com/Byron/crates-index-diff-rs/commit/05bffd68e3411ca4327c7a905c9a4e562c9c2eb3))
    - try without index-bare cache ([`b11f6df`](https://github.com/Byron/crates-index-diff-rs/commit/b11f6df25e6a12e4835f32b739b7c7fd890d6c23))
    - custom code to get cmake onto osx ([`4860906`](https://github.com/Byron/crates-index-diff-rs/commit/48609062e30407ea29afa5ddccea78e7c94f20bd))
    - let's hope it will actually run now ([`a052f41`](https://github.com/Byron/crates-index-diff-rs/commit/a052f41cd198b7342b14aba186a955067a9a7241))
    - customizations for us ([`c4bf948`](https://github.com/Byron/crates-index-diff-rs/commit/c4bf948b5e2c5590e58a134a3003acde7738e42d))
    - using openssl file as basis ([`accb62d`](https://github.com/Byron/crates-index-diff-rs/commit/accb62d00618204e76659ecb4d31e8a04291bdc5))
    - test osx as well ([`b0f19b0`](https://github.com/Byron/crates-index-diff-rs/commit/b0f19b0a5d754cd9153b30ca9b363fa9534777da))
    - don't enforce branches ([`601dc2d`](https://github.com/Byron/crates-index-diff-rs/commit/601dc2d7ca9bd87f440455e1eb3698c6e2f0227d))
    - use quick-error ([`ef53496`](https://github.com/Byron/crates-index-diff-rs/commit/ef5349677d53ab2d31c921b853766a8aa21f59fa))
    - move crate into own module ([`97a5b81`](https://github.com/Byron/crates-index-diff-rs/commit/97a5b819df11a454f5154ae3edd5e4a03192c0e1))
    - better diff.print implementation ([`81c95f2`](https://github.com/Byron/crates-index-diff-rs/commit/81c95f2a9014ace72e74da64ddb4840197d7bbec))
    - simplify fetch-changes error handling ([`988b668`](https://github.com/Byron/crates-index-diff-rs/commit/988b66860a96411f9c263c582159066970ef9265))
    - much nicer Crate from json ([`caa5879`](https://github.com/Byron/crates-index-diff-rs/commit/caa58790f61ec4e450dadbf737a0bc3224f8c0b4))
    - allow to change seen-ref name ([`56d416a`](https://github.com/Byron/crates-index-diff-rs/commit/56d416aae569d8dbcd568428a7489072eb749249))
    - remove duplicate ([`42e6803`](https://github.com/Byron/crates-index-diff-rs/commit/42e68031d586109efe7d9567279af4d4bf7ac0be))
    - be compatible with docs.rs ([`a4e6e7e`](https://github.com/Byron/crates-index-diff-rs/commit/a4e6e7efef4bf7d6863eeade1b974a350d08f4a6))
    - set envvars propery ([`b784f1c`](https://github.com/Byron/crates-index-diff-rs/commit/b784f1cb3655a2ebef7845ed62cd5c52ab31d46e))
    - fix keywords ([`787bdac`](https://github.com/Byron/crates-index-diff-rs/commit/787bdacfa2c1191e121d4d5a6c8e3a09c19bf684))
    - show backtrace ([`ed7ca36`](https://github.com/Byron/crates-index-diff-rs/commit/ed7ca366454a0c99698f18beb5955cd6606c7e1e))
    - better error handling ([`b0dd277`](https://github.com/Byron/crates-index-diff-rs/commit/b0dd2770a979b278e8be4432418367c6d9620c67))
    - attempt of fetch_changes implementation ([`708d9c0`](https://github.com/Byron/crates-index-diff-rs/commit/708d9c0680b797026da731bc9a9874ac71bc125b))
    - allow direct usage of objects as well ([`eee5134`](https://github.com/Byron/crates-index-diff-rs/commit/eee513476d300c294272e0c23348cdbc1009f008))
    - fetch_changes() ([`d06c4a0`](https://github.com/Byron/crates-index-diff-rs/commit/d06c4a0bb526eafb13a61951cd34c24febd10797))
    - remove debug prints ([`24c1c81`](https://github.com/Byron/crates-index-diff-rs/commit/24c1c81d7e1287ed83308f4f8ad795dfca4de834))
    - assure we can handle the entire index ([`97b417a`](https://github.com/Byron/crates-index-diff-rs/commit/97b417ad71b3070e3663d1ce6d998f24a0bf1365))
    - support for unyanking ([`2ef9c02`](https://github.com/Byron/crates-index-diff-rs/commit/2ef9c028812134af6bf23f72a4ea9850c407a06a))
    - handle yanked files ([`8048a2c`](https://github.com/Byron/crates-index-diff-rs/commit/8048a2cf00618d669c9176b0e94353dd1cfa9011))
    - remove error handling in makefile ([`9ef08fa`](https://github.com/Byron/crates-index-diff-rs/commit/9ef08fa5d1517883cb006fda522d020767f8d081))
    - see why makefile is not working ([`11a006b`](https://github.com/Byron/crates-index-diff-rs/commit/11a006bc230d098ad8ee282069b2544c8187d14a))
    - to work with GNU make ([`42e1290`](https://github.com/Byron/crates-index-diff-rs/commit/42e1290ea75ddf57948b6f6acd23c2c17544db06))
    - now seeing the first added crates ([`887c088`](https://github.com/Byron/crates-index-diff-rs/commit/887c088495ef78e21ca88091963dbfd0661e08ec))
    - found an illegal-instruction error ([`607d747`](https://github.com/Byron/crates-index-diff-rs/commit/607d747c9306ad0921a6d4166ecf151bb9a39479))
    - intermediate crate info from hunk ([`3ce91e8`](https://github.com/Byron/crates-index-diff-rs/commit/3ce91e89b520b6497eceb961065e8898c5aba883))
    - automate running tests quickly ([`8801ec2`](https://github.com/Byron/crates-index-diff-rs/commit/8801ec2d1d718eb73200d29ff23a958b5b1ba9d7))
    - support for commit'ishs for diffs ([`e451067`](https://github.com/Byron/crates-index-diff-rs/commit/e451067a939a848082def317e1cceb487910aba2))
    - finally the correct refspec ([`f89651b`](https://github.com/Byron/crates-index-diff-rs/commit/f89651be89638e8210b74dc8843d5985d28e842b))
    - first traversal method ([`d49f62f`](https://github.com/Byron/crates-index-diff-rs/commit/d49f62fa41dbba9278ec2080ae2b91f72dc6834e))
    - cmake binary installation ([`d0d4307`](https://github.com/Byron/crates-index-diff-rs/commit/d0d43071a3fdcd795a2460b71363545dbf5a2a89))
    - use different cmake source ([`f9f67ef`](https://github.com/Byron/crates-index-diff-rs/commit/f9f67efd7ca12c34d4bbfa171b8604f206d686b4))
    - cmake upgrade via before_script ([`a978136`](https://github.com/Byron/crates-index-diff-rs/commit/a97813620f5990f4d62337e7a6a8c3b3aadd05ef))
    - see if a custom source will get latest cmake ([`77bbd20`](https://github.com/Byron/crates-index-diff-rs/commit/77bbd20132b7f42dd79b55f86c000f21618d4c39))
    - enforce latest cmake ([`2d0d5d9`](https://github.com/Byron/crates-index-diff-rs/commit/2d0d5d94193d5a65cdac700d811e9d75e849a7c9))
    - test against all versions of rust ([`094c788`](https://github.com/Byron/crates-index-diff-rs/commit/094c788f0b9ebd7beda17a8a7ee71d88ebbaad71))
    - simplify travis ([`f9d531a`](https://github.com/Byron/crates-index-diff-rs/commit/f9d531a63269e8e236489c9a7bb56a6bafdd9eeb))
    - first clone on demand ([`fdf281e`](https://github.com/Byron/crates-index-diff-rs/commit/fdf281ee3b301ab370fe1d3ea6aeb1dd5e0536d1))
    - first failing test ([`1c0b2c0`](https://github.com/Byron/crates-index-diff-rs/commit/1c0b2c0a3723afd587779880337f5a5516c063f1))
    - first commit ([`f1568c0`](https://github.com/Byron/crates-index-diff-rs/commit/f1568c02652781163055bcbc18bc4af0e6914cd2))
</details>

