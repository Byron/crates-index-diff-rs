# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

This release adds support for obtaining changes in the correct order by automatically looking at changes one commit at a time
while handling squashed indices gracefully. It take about 20 seconds to catch up with all commits done in the crates index for
2 whole days and takes much longer than a similar unorderd acquisition of changes, but should be well worth it in situations
where changes are fetched more often than that.

The baseline validation to assure correctness was improved to also assert the `yanked` state of crates. Furthermore, it is 
regularly run by CI, on the real crates-index, and in a more realistic manner mixing both unordered and ordered change requests.

The API is mostly the same, but has a few breaking changes to deal with order selection.

Last but not least, the user can now configure the HTTP backend to use, which allows for a pure-Rust
build as well.

### Chore

 - <csr-id-d91afc930e833f4eb90f64971200d691662f9b0d/> a pipeline to validate basic assumptions are stil fulfilled.
   Running stress-test like baseline tests regularly should help us
   assure that `crates-index-diff` operates as it should against a
   real crates-index.

### New Features

 - <csr-id-87e49b59c3a1542bee9c2965e062a8045748e821/> baseline validation now validates ordered and unordered mode.
 - <csr-id-81c6dd2a2413d2556284ef188c06059c1177bc42/> greatly improve performance and realism of `baseline-atomic` test.
   We now set a fixed amount of 'big' steps along with one of those chunks
   being a range where the step-size is one commit at a time, which
   might be the way changes are obtained in the future.
 - <csr-id-4dd4a4c86f710fad582e4cf82f799384e42921d9/> baseline also validates the `yanked` status.
   That way we assure that the state we are interested in is indeed
   communicated perfectly through the events we emit.

### New Features (BREAKING)

 - <csr-id-133f2f5db418470e6ab4537ebd9a123f33e5fe7b/> Support for in-order changes via `changes_between_ancestor_commits()`.
   This improvement also makes available an enum to select `Order`
   in higher-level methods like `peek_changes_with_options()`.
   
   We also add `peek_changes_ordered()` and `changes_ordered()` for convenience.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#30](https://github.com/Byron/crates-index-diff-rs/issues/30)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#30](https://github.com/Byron/crates-index-diff-rs/issues/30)**
    - baseline validation now validates ordered and unordered mode. ([`87e49b5`](https://github.com/Byron/crates-index-diff-rs/commit/87e49b59c3a1542bee9c2965e062a8045748e821))
    - greatly improve performance and realism of `baseline-atomic` test. ([`81c6dd2`](https://github.com/Byron/crates-index-diff-rs/commit/81c6dd2a2413d2556284ef188c06059c1177bc42))
    - baseline also validates the `yanked` status. ([`4dd4a4c`](https://github.com/Byron/crates-index-diff-rs/commit/4dd4a4c86f710fad582e4cf82f799384e42921d9))
 * **Uncategorized**
    - Merge branch 'baseline-improvements' ([`a80c7fa`](https://github.com/Byron/crates-index-diff-rs/commit/a80c7faab2770dfc6b4593a02c5c897f055c5fe5))
    - Support for in-order changes via `changes_between_ancestor_commits()`. ([`133f2f5`](https://github.com/Byron/crates-index-diff-rs/commit/133f2f5db418470e6ab4537ebd9a123f33e5fe7b))
    - a pipeline to validate basic assumptions are stil fulfilled. ([`d91afc9`](https://github.com/Byron/crates-index-diff-rs/commit/d91afc930e833f4eb90f64971200d691662f9b0d))
</details>

## 14.0.0 (2022-11-21)

<csr-id-4d53b045ec3a006205b466ea051c7e1030ea981c/>
<csr-id-c0c01bb5d63c6d469a298e157cd4063853ecc50e/>

A massive release that increases performance up to 10x for diffing repositories[^1] and correctness at the same time. This release
wouldn't have been possible without the herculean efforts of [Pascal Kuthe](https://github.com/Byron/crates-index-diff-rs/pull/29) and I am grateful 
for his contribution. Thank you!

[^1]: Needs to build with `git-repository/max-performance` and setup a pack cache, for example with `GITOXIDE_PACK_CACHE_MEMORY=1g <you-application>`

### Other

 - <csr-id-4d53b045ec3a006205b466ea051c7e1030ea981c/> try to rewrite delegate to be map based…
   …but besides completely failing the normalization test which I don't
   understand, it also doesn't manage to get the correct amount of
   versions.
 - <csr-id-c0c01bb5d63c6d469a298e157cd4063853ecc50e/> try to chunk up baseline, but changes do not line up.
   When stepping through the changes in multiple steps, we end up with
   more crates then there are even though we identify them by
   checksum and consider deletions. Yanking doesn't remove them from
   the iteration either.

### New Features

<csr-id-4d53b045ec3a006205b466ea051c7e1030ea981c/>
<csr-id-c0c01bb5d63c6d469a298e157cd4063853ecc50e/>

 - <csr-id-6f5b12a35d0ae6b2bcb05c4f42153ec8fa4f37a2/> a `max-performance` feature to tune `git-repository`.
   The performance difference is rather drastic at about 2.5x, and
   definitely worth having if there is no compatibility issue
   due to shared C dependencies in the same binary.
   
   Additionally we setup the makefile to use big object caches
   to avoid having to decompress the same object too often, accelerating
   the diffing process about 4x, for a total of 10x performance boost.

### Changed (BREAKING)

 - <csr-id-b538ad6ad9c6b11354583f32986b16907de7f4f4/> `Change::Deleted` variant now has `versions` field to include all deleted versions.
   That way it doesn't degenerate any information, previously the exact
   version information was lost.
   
   Not doing so helps to be able to reproduce the current state by
   aggregating all changes.

### New Features (BREAKING)

 - <csr-id-f9be536b089199460330cf0ad6d6a74d8813a9cf/> Reduce heap-allocations `CrateVersion` type and sub-types.
   This improves performance slightly when dealing with a lot of versions,
   like when all versions are obtained from the beginning of time.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#26](https://github.com/Byron/crates-index-diff-rs/issues/26)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#26](https://github.com/Byron/crates-index-diff-rs/issues/26)**
    - refactor ([`b42ac1e`](https://github.com/Byron/crates-index-diff-rs/commit/b42ac1e055bb8264804c40a2fe436e45850e9422))
    - revert previous to FAILed commits ([`3b52cfd`](https://github.com/Byron/crates-index-diff-rs/commit/3b52cfdf74fc1b38d604a79e83dc0b0de1f61843))
    - try to rewrite delegate to be map based… ([`4d53b04`](https://github.com/Byron/crates-index-diff-rs/commit/4d53b045ec3a006205b466ea051c7e1030ea981c))
    - try to chunk up baseline, but changes do not line up. ([`c0c01bb`](https://github.com/Byron/crates-index-diff-rs/commit/c0c01bb5d63c6d469a298e157cd4063853ecc50e))
    - refactor ([`097209c`](https://github.com/Byron/crates-index-diff-rs/commit/097209c3fe54d56fd908907a54ab07268ba8804b))
    - Now the baseline result is the same. ([`02cdb2e`](https://github.com/Byron/crates-index-diff-rs/commit/02cdb2ee5425e3f88d1ad8720c6ac2f3f247716d))
    - make baseline work with CI ([`b9a1850`](https://github.com/Byron/crates-index-diff-rs/commit/b9a1850b6723c26d3af359cce9163747d30e8874))
    - a baseline test which shows that we cannot reproduce the status quo with changes just yet. ([`3fcf96b`](https://github.com/Byron/crates-index-diff-rs/commit/3fcf96be95647bf9e66c85a33b52c7e74ccc9cce))
    - `Change::Deleted` variant now as `versions` to include all deleted versions. ([`b538ad6`](https://github.com/Byron/crates-index-diff-rs/commit/b538ad6ad9c6b11354583f32986b16907de7f4f4))
    - Reduce heap-allocations `CrateVersion` type and sub-types. ([`f9be536`](https://github.com/Byron/crates-index-diff-rs/commit/f9be536b089199460330cf0ad6d6a74d8813a9cf))
    - layout baseline for exhaustive test against the latest available index ([`7e9d3cd`](https://github.com/Byron/crates-index-diff-rs/commit/7e9d3cd25afa27bee80a382dfe61792a99ed0f35))
 * **Uncategorized**
    - Release crates-index-diff v14.0.0 ([`dfaf1be`](https://github.com/Byron/crates-index-diff-rs/commit/dfaf1beb292dea55d40d5d8d6d5e0cba93f82a69))
    - prepare changelog prior to release ([`a93ba40`](https://github.com/Byron/crates-index-diff-rs/commit/a93ba40269cec0f02c041b299952df22b3010736))
    - Merge branch 'fix-diff' ([`ec9842a`](https://github.com/Byron/crates-index-diff-rs/commit/ec9842ac8861a55cb51ed28caeeea5e0a18757f3))
    - refactor ([`bd3bc22`](https://github.com/Byron/crates-index-diff-rs/commit/bd3bc220ab28679cb4ce81376aeb4088b5053279))
    - remove unnecessary unsafe code ([`1b5684f`](https://github.com/Byron/crates-index-diff-rs/commit/1b5684fe702545d120877852e64ffdb800bbc2e4))
    - a `max-performance` feature to tune `git-repository`. ([`6f5b12a`](https://github.com/Byron/crates-index-diff-rs/commit/6f5b12a35d0ae6b2bcb05c4f42153ec8fa4f37a2))
    - improve baseline tests to be more practical ([`bae80b0`](https://github.com/Byron/crates-index-diff-rs/commit/bae80b04ecd97ac37c12f90f5cd60a480b9bea6a))
    - add baseline tests that steps trough each commit individually ([`a377ca4`](https://github.com/Byron/crates-index-diff-rs/commit/a377ca47627e0c4449f0410d2887afed4d07d634))
    - perform an unordered comparison instead of using a linear edit-sequence ([`8256cbb`](https://github.com/Byron/crates-index-diff-rs/commit/8256cbbd3651073394d6aa9de38c734618df9102))
    - Merge branch 'complete-baseline' ([`61c6272`](https://github.com/Byron/crates-index-diff-rs/commit/61c62723520aa6257adf75cf4e3558187f986844))
    - thanks clippy ([`249d141`](https://github.com/Byron/crates-index-diff-rs/commit/249d14118f8716ffd83adc6559edf41e89b9c4a8))
</details>

## 13.0.3 (2022-11-18)

### Bug Fixes

 - <csr-id-51c5109c30d42eabf0a4ee4c8272aaec3275b556/> assure differences are handled exhaustively.
   Previously it was possible to have multiple diffs in one crate
   distributed over multiple commits to rightfully show up as multiple
   hunks of modified and added lines only register the modified lines,
   not the new ones (or the deleted ones for that matter).
   
   This would cause updates or removals to be missed.
   
   Now hunks of changes are exhaused properly, fixing [the issue](https://github.com/Byron/crates-index-diff-rs/issues/26).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 9 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#26](https://github.com/Byron/crates-index-diff-rs/issues/26)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#26](https://github.com/Byron/crates-index-diff-rs/issues/26)**
    - prepare changelog prior to release ([`46d94b5`](https://github.com/Byron/crates-index-diff-rs/commit/46d94b5120364aa5a285492b8fce543e6487fe98))
    - refactor ([`87678db`](https://github.com/Byron/crates-index-diff-rs/commit/87678db79a798c17575678c233dc9d21f4e16d70))
    - thanks clippy ([`425fd57`](https://github.com/Byron/crates-index-diff-rs/commit/425fd57b8223d8fb35503b7aee06f16c403d1ea5))
    - refactor ([`d059360`](https://github.com/Byron/crates-index-diff-rs/commit/d0593607c76e7b31fd8ac91a0068de128e6bfaf4))
    - assure differences are handled exhaustively. ([`51c5109`](https://github.com/Byron/crates-index-diff-rs/commit/51c5109c30d42eabf0a4ee4c8272aaec3275b556))
    - reproduce issue ([`69c8f43`](https://github.com/Byron/crates-index-diff-rs/commit/69c8f43829166949e7afeb8d42c8076480bc3c08))
    - Add test fixtures for reproduction ([`462d44f`](https://github.com/Byron/crates-index-diff-rs/commit/462d44fd019ea2544c070163a2ec2839f9c57b4d))
 * **Uncategorized**
    - Release crates-index-diff v13.0.3 ([`1d06ee9`](https://github.com/Byron/crates-index-diff-rs/commit/1d06ee90b51d1147fc6cb21370744799ffd9a512))
    - Merge branch 'fix-26' ([`7ea3d6e`](https://github.com/Byron/crates-index-diff-rs/commit/7ea3d6e89e4804ff1cdfa664c9454f433ca35dc8))
</details>

## 13.0.2 (2022-11-08)

### Features

- Switch diff implementation to [`imara-diff`](https://github.com/pascalkuthe/imara-diff) for performance, but also to get off the broken `git-repository` v0.25 release.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 28 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v13.0.2 ([`3708131`](https://github.com/Byron/crates-index-diff-rs/commit/3708131ce4eb40bb500ab430bd10c5142f423cf8))
    - prepare changelog prior to release ([`adecda5`](https://github.com/Byron/crates-index-diff-rs/commit/adecda5290674442baf4315772103200fdb7ad8b))
    - Merge branch 'git-repository-upgrade' ([`b28b7ca`](https://github.com/Byron/crates-index-diff-rs/commit/b28b7ca065bec9ff894ee9c0639ae441564f56fd))
    - upgrade to git-repository v0.27 ([`377065e`](https://github.com/Byron/crates-index-diff-rs/commit/377065ecbbbe8dfa0817873fd03b80f9e70bb7aa))
    - all tests are green ([`6b47427`](https://github.com/Byron/crates-index-diff-rs/commit/6b47427e6fa98b01bfda5cd7508179c95203647a))
    - one more test bytes the dust, but… ([`62cb6eb`](https://github.com/Byron/crates-index-diff-rs/commit/62cb6ebd66802d6500b9cb52e94cd1581ad8fe9b))
    - fix most of the tests ([`276e726`](https://github.com/Byron/crates-index-diff-rs/commit/276e726221bab2bea2e0d06ff396f757df74d393))
    - A first stab at getting the diff back to work with `imara-diff` ([`5573529`](https://github.com/Byron/crates-index-diff-rs/commit/5573529fa44c2c1e7e108c00c0784f301c599d61))
    - try to rewrite explicit trait impl as closure, same lifeimte issues. ([`2809c75`](https://github.com/Byron/crates-index-diff-rs/commit/2809c75cff185190d6f1075e10c1d5f1668200ed))
    - upgrade to `git-repoitory v0.26`, fail to make diff compile. ([`3f47ec3`](https://github.com/Byron/crates-index-diff-rs/commit/3f47ec38b4cfe1ed2b3ec2cb88d31e4598dafe67))
</details>

## 13.0.1 (2022-10-11)

### Bug Fixes

 - <csr-id-a6975ce303385cb124a2409d2900a1f2aa278225/> rename `init::Error2` to `init::Error`.
   This name was left by mistake and shouldn't have made it into the
   release.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v13.0.1 ([`acd06a1`](https://github.com/Byron/crates-index-diff-rs/commit/acd06a154e3a50e76a3a29c14fa64880a734e6f3))
    - rename `init::Error2` to `init::Error`. ([`a6975ce`](https://github.com/Byron/crates-index-diff-rs/commit/a6975ce303385cb124a2409d2900a1f2aa278225))
</details>

## 13.0.0 (2022-10-10)

This release drops `git2` in favor of `gitoxide`.

### Important note for users of ssh:// index urls

Advanced `git2` based credential configuration isn't supported
anymore until `gitoxide` catches up. It generally implements all
configuration options that are relevant for `git` and fully implements
HTTP based authentication, but is probably lacking in regard to 
non-default ssh configuration.

If that's a problem, prefer staying with the v12.X line.

### Changed (BREAKING)

 - <csr-id-4cedf27d510ff9031bf9f142ecbb3788a6337f8c/> remove `git2` in favor of `gitoxide`.
   `gitoxide` is now used for cloning and fetching as well.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 6 calendar days.
 - 11 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v13.0.0 ([`2b08485`](https://github.com/Byron/crates-index-diff-rs/commit/2b08485ca4739aff0e3f6619f0ae92c9d032557f))
    - prepare changelog  prior to release ([`35b1ce7`](https://github.com/Byron/crates-index-diff-rs/commit/35b1ce77a4ee3ad5ebfd494926e5881c9c8ea473))
    - Merge branch 'remove-git2' ([`ab529ad`](https://github.com/Byron/crates-index-diff-rs/commit/ab529adbcd85691958297cf451898cb1bece63cd))
    - switch to released version of git-repository (v0.25) ([`533bfab`](https://github.com/Byron/crates-index-diff-rs/commit/533bfabc36d4c1d515c30f4d16733b62dbac02b9))
    - fix makefile ([`7cede70`](https://github.com/Byron/crates-index-diff-rs/commit/7cede70cc829919ea4d3b40f61e2a7ee8f6057a6))
    - remove `git2` in favor of `gitoxide`. ([`4cedf27`](https://github.com/Byron/crates-index-diff-rs/commit/4cedf27d510ff9031bf9f142ecbb3788a6337f8c))
    - all tests pass, time to remove git2 ([`6975d67`](https://github.com/Byron/crates-index-diff-rs/commit/6975d671fa9f11f7f11f520705b4832a080bb3b7))
    - use gitoxide based cloning on demand in tests, one failure ([`1774f94`](https://github.com/Byron/crates-index-diff-rs/commit/1774f94e8de0939ca66df8fc0240355967be5699))
    - Add a way to clone using `gitoxide` without removing git2 just yet. ([`878abe9`](https://github.com/Byron/crates-index-diff-rs/commit/878abe9071b8d36df496206a0eda204aa133c274))
    - Add `fetch_changes_with_options()` using gitoxide ([`900ae56`](https://github.com/Byron/crates-index-diff-rs/commit/900ae56286894ddcdb371bf3bea55a7ba4dfba72))
    - Use gitoxide for fetching in test ([`00e6875`](https://github.com/Byron/crates-index-diff-rs/commit/00e6875209b059797fccb538dd97122e10803d20))
    - sketch a new peek method that uses gitoxide to fetch things ([`759ff30`](https://github.com/Byron/crates-index-diff-rs/commit/759ff30ad771deefa3d226badea72796c74f3096))
</details>

## 12.1.0 (2022-09-29)

### New Features

 - <csr-id-74866b44ada127894b63969818e64564a294c8d0/> re-export `git-repository` as `git`
   This makes type conversions possible where needed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v12.1.0 ([`ad2f6d9`](https://github.com/Byron/crates-index-diff-rs/commit/ad2f6d9f6e2bf3ac7f0a5db2a2bdcbdc92a0744a))
    - re-export `git-repository` as `git` ([`74866b4`](https://github.com/Byron/crates-index-diff-rs/commit/74866b44ada127894b63969818e64564a294c8d0))
</details>

## 12.0.0 (2022-09-29)

This release is v11.2, but correctly indicates the **breaking change** introduced by upgrading
`git-repository` to v0.24, which fixes [#23](https://github.com/Byron/crates-index-diff-rs/issues/23).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#23](https://github.com/Byron/crates-index-diff-rs/issues/23)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#23](https://github.com/Byron/crates-index-diff-rs/issues/23)**
    - update changelog prior to release ([`6017b3f`](https://github.com/Byron/crates-index-diff-rs/commit/6017b3fe3f46079e86181d1a7107cd77dd1ab797))
    - update version to 12.0 to indicate breaking change via git-repository ([`1fcac4b`](https://github.com/Byron/crates-index-diff-rs/commit/1fcac4b92ec7d2ef90206dabeebc7d07322b0723))
 * **Uncategorized**
    - Release crates-index-diff v12.0.0 ([`ee27555`](https://github.com/Byron/crates-index-diff-rs/commit/ee275556b2bac9845b9dc71165df8caf51bad6a5))
</details>

## 11.2.0 (2022-09-20)

### New Features

 - <csr-id-91593970b6989b93a48138ca35a799808e99afd0/> upgrade to `git-repository` v0.24 to simplify diff implementation.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v11.2.0 ([`7e420d2`](https://github.com/Byron/crates-index-diff-rs/commit/7e420d284099f195f316429d0ac9ca2262cf881c))
    - Use the latest diff API to obtain line diffs. ([`300e966`](https://github.com/Byron/crates-index-diff-rs/commit/300e966f905feb75d8ecc721ae479f0a5e68e681))
    - upgrade to `git-repository` v0.24 to simplify diff implementation. ([`9159397`](https://github.com/Byron/crates-index-diff-rs/commit/91593970b6989b93a48138ca35a799808e99afd0))
    - remove accidentally added example ([`cd23a28`](https://github.com/Byron/crates-index-diff-rs/commit/cd23a28c45b4d3e6f6ac2fce1034a03eee0ac65f))
</details>

## 11.1.6 (2022-09-14)

### Bug Fixes

 - <csr-id-4ce0021433ba2f4636ff97156ce323c1d8c6042e/> Ignore directory deletions.
   Previously deleted directories would be picked up as crate deletions,
   with the crate name being the deleted directory.
   
   Now only file deletions will be assumed to be crate deletions.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 11 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#20](https://github.com/Byron/crates-index-diff-rs/issues/20)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#20](https://github.com/Byron/crates-index-diff-rs/issues/20)**
    - Ignore directory deletions. ([`4ce0021`](https://github.com/Byron/crates-index-diff-rs/commit/4ce0021433ba2f4636ff97156ce323c1d8c6042e))
 * **Uncategorized**
    - Release crates-index-diff v11.1.6 ([`3a1ecd1`](https://github.com/Byron/crates-index-diff-rs/commit/3a1ecd183e695772a602d32b8e512eb501fa1201))
</details>

## 11.1.5 (2022-09-02)

### Bug Fixes

 - <csr-id-8af61f2a20eee72b0e53ae3b6ce22a3a9d52546c/> Ignore all changed files with an extension.
   There are non-crate files that as far as we know all have file
   extensions, as opposed to the crate files we are interested in, which do
   not.
   
   Thus skipping all files with extension helps us to get past the initial
   commit which includes such files, like `.github/*.yml`.
   
   Related to https://github.com/rust-lang/docs.rs/pull/1807#issuecomment-1235158502

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v11.1.5 ([`516a779`](https://github.com/Byron/crates-index-diff-rs/commit/516a779f861abeb3972ec721512fe424ab70d8e6))
    - Ignore all changed files with an extension. ([`8af61f2`](https://github.com/Byron/crates-index-diff-rs/commit/8af61f2a20eee72b0e53ae3b6ce22a3a9d52546c))
</details>

## 11.1.4 (2022-09-02)

### Bug Fixes

 - <csr-id-ab6e46ced56d1aaed22b5619cfbc5c131a93ba32/> improve error descriptions and provide details when decoding of crate versions fails.
   It was suggested in this comment:
   https://github.com/rust-lang/docs.rs/pull/1807#issuecomment-1234825498 ,
   and if marker references aren't as expected it's possible to diff lines
   that are not actually crate versions.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v11.1.4 ([`9d41b20`](https://github.com/Byron/crates-index-diff-rs/commit/9d41b200f5bec12953d62fc4ca892f38b6d28ac1))
    - improve error descriptions and provide details when decoding of crate versions fails. ([`ab6e46c`](https://github.com/Byron/crates-index-diff-rs/commit/ab6e46ced56d1aaed22b5619cfbc5c131a93ba32))
    - fix rev-spec ([`83e91f5`](https://github.com/Byron/crates-index-diff-rs/commit/83e91f56e4f93b6de5210e6a5f6d80afaeba4e6d))
</details>

## 11.1.3 (2022-09-01)

### Bug Fixes

 - <csr-id-a430c03d950d073084e3555cb264fa5c416b8ded/> switch git2 back to v0.14 - v0.15 is a breaking change…
   …for everyone who uses git2 as direct dependency as well due to
   libgit2-sys.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v11.1.3 ([`d46402a`](https://github.com/Byron/crates-index-diff-rs/commit/d46402ab2ed4e65c7b1c03635ed39ea68d31ffe7))
    - switch git2 back to v0.14 - v0.15 is a breaking change… ([`a430c03`](https://github.com/Byron/crates-index-diff-rs/commit/a430c03d950d073084e3555cb264fa5c416b8ded))
</details>

## 11.1.2 (2022-09-01)

### Bug Fixes

 - <csr-id-23a66b9da25ef40d4e545bed028788f83836a584/> make fetches work again by using safe-performance mode of `git-repository`.
   This fixes the 'zlib stream broken' issue when fetching crates.io
   changes which was caused by `git-repository` configuring for
   max-performance by default, which affects a crate used by `git2` as
   well. For some reason, changing to `zlib-ng` as backend wasn't taken
   kindly by `libgit2` causing it to fail after a short while of receiving
   a pack from the remote.
   
   The fix avoids making such modifications to the zlib crate, allowing
   both crates, `git-repository` and `git2` to co-exist in the same
   dependency tree.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v11.1.2 ([`5b2e3ce`](https://github.com/Byron/crates-index-diff-rs/commit/5b2e3ce028c043d8cc8c0e100f225626d5f11654))
    - Merge branch 'fix-zlib-stream-error' ([`ac83e7a`](https://github.com/Byron/crates-index-diff-rs/commit/ac83e7a5afaa7ae238f441dccfdbcb97edc5edc3))
    - make fetches work again by using safe-performance mode of `git-repository`. ([`23a66b9`](https://github.com/Byron/crates-index-diff-rs/commit/23a66b9da25ef40d4e545bed028788f83836a584))
    - refactor ([`e93f1c6`](https://github.com/Byron/crates-index-diff-rs/commit/e93f1c66fd4c18de60f6fe75f913e8a1d7968a29))
    - Upgrade to latest git2 version ([`7616db2`](https://github.com/Byron/crates-index-diff-rs/commit/7616db2a4022fdf97cab00ed298242de46291f23))
    - properly parameterize script so it's obvious what is what ([`02f715e`](https://github.com/Byron/crates-index-diff-rs/commit/02f715e0b6b74559b8702234798c36eae510f6de))
    - Make test-lookup independent of prior commits ([`4b7fc6e`](https://github.com/Byron/crates-index-diff-rs/commit/4b7fc6e54336b642f0739b8d7845a9dba2346209))
</details>

## 11.1.1 (2022-08-31)

### Bug Fixes

 - <csr-id-28de9d4a6385bd495dbf93ef0d2b58e00e993104/> Consider all crates yanked if 'yanked = true'.
   Previously, due to a missing test, a bug snuck in that would assume
   that all lines in an added files must be new versions, marking them
   as `Change::Added`. This ignored the fact that any line could also
   carry a yanked crate, misrepresenting them.
   
   This is now fixed, and yanked crates generally show up as
   `Change::Yanked`.
   
   One might take this as a hint that diffentiating by yank status might
   not be that useful after all as it doesn't scale that well. Maybe
   a future version changes how `Change` is represented.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#19](https://github.com/Byron/crates-index-diff-rs/issues/19)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#19](https://github.com/Byron/crates-index-diff-rs/issues/19)**
    - Consider all crates yanked if 'yanked = true'. ([`28de9d4`](https://github.com/Byron/crates-index-diff-rs/commit/28de9d4a6385bd495dbf93ef0d2b58e00e993104))
    - Add failing test ([`89378e1`](https://github.com/Byron/crates-index-diff-rs/commit/89378e10b395cb1c0d963557d46568fda7b49f7b))
 * **Uncategorized**
    - Release crates-index-diff v11.1.1 ([`708288f`](https://github.com/Byron/crates-index-diff-rs/commit/708288f52f26ecff03ddc89ea792be1a765b8b85))
</details>

## 11.1.0 (2022-08-30)

### New Features

 - <csr-id-09489ab888124954119c70a5828f0e1011198253/> make the name of the remote configurable.
   This is primarily used in testing and we try even harder to make it fail
   to deal with squashed remote references, but it seems to work fine.
   
   For good measure, now using `+` in the refspec to assure it forces
   an update.

### Bug Fixes

 - <csr-id-cdcac4127490713de60e19d007ebcac0a2459c0d/> assure refs can be reet when fetching to support squashing.
   Previously tests didn't replicate this, now they do.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#16](https://github.com/Byron/crates-index-diff-rs/issues/16)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#16](https://github.com/Byron/crates-index-diff-rs/issues/16)**
    - fix assertion message ([`51559b8`](https://github.com/Byron/crates-index-diff-rs/commit/51559b87a7c6b28b626e74b64445335d9a9f813a))
 * **Uncategorized**
    - Release crates-index-diff v11.1.0 ([`01ca770`](https://github.com/Byron/crates-index-diff-rs/commit/01ca770ba0e2fd14994c6ca5ba1ecfa6c0349fa3))
    - make the name of the remote configurable. ([`09489ab`](https://github.com/Byron/crates-index-diff-rs/commit/09489ab888124954119c70a5828f0e1011198253))
    - assure refs can be reet when fetching to support squashing. ([`cdcac41`](https://github.com/Byron/crates-index-diff-rs/commit/cdcac4127490713de60e19d007ebcac0a2459c0d))
    - Add test to validate we can deal with squashed indices ([`ed4ba38`](https://github.com/Byron/crates-index-diff-rs/commit/ed4ba38ce3875372c35cbd9476b1e922065f2f78))
</details>

## 11.0.0 (2022-08-28)

### Changed (BREAKING)

 - <csr-id-2d3a182819077a1fe068cb16fdfeceed2f6882da/> Use `gitoxide` `Repository` instead of `git2::Repository`
   This comes with plenty of changes to the API of the
   `last_seen_reference()` and to the lower-level methods that take
   object ids (now `git::hash::ObjectId`.
   
   Note that `git2` is still used internally for fetching and cloning.
   This change was made to assure that at no time there are two open
   repositories (once for git2, once for `gitoxide`), as this has the
   potential to double resource usage for file handles.
 - <csr-id-07f4b6c022ae8c48907250314db3a9eeb59ae89e/> move `CloneOptions` into `index` module.
   The `index` module is now public for that reason.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 39 commits contributed to the release over the course of 1 calendar day.
 - 97 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#16](https://github.com/Byron/crates-index-diff-rs/issues/16)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#16](https://github.com/Byron/crates-index-diff-rs/issues/16)**
    - update to latest release of `gitoxide` ([`23e14af`](https://github.com/Byron/crates-index-diff-rs/commit/23e14af509e72efaa65215f3f6f88166e32dbeb0))
    - and normalization works now ([`eb148e5`](https://github.com/Byron/crates-index-diff-rs/commit/eb148e512bc29cdc81f4eb35bda0a1819d9abd69))
    - first stab at normalization can reduce 245 version, but… ([`ae3f971`](https://github.com/Byron/crates-index-diff-rs/commit/ae3f971e627d491e239220bb5ba15a89c026302e))
    - adapt to changes in git-repository ([`68ff142`](https://github.com/Byron/crates-index-diff-rs/commit/68ff142961f4afc2d2d31b4d457047a6db321156))
    - remove git2 verion of the diff algorithm ([`371b512`](https://github.com/Byron/crates-index-diff-rs/commit/371b51293047b41beac82be7a9e9d1bd43fd5d7a))
    - refactor ([`3749220`](https://github.com/Byron/crates-index-diff-rs/commit/374922041710eb9ace92a7a319c2c407a4897baa))
    - refactor ([`7cee17e`](https://github.com/Byron/crates-index-diff-rs/commit/7cee17eccd74757277049e92c8074ee41cceedaf))
    - all diff-tests pass like before ([`9ba7921`](https://github.com/Byron/crates-index-diff-rs/commit/9ba79212042a501e1cae21ce28daea2a3637a383))
    - handle modifications and yanks ([`3416414`](https://github.com/Byron/crates-index-diff-rs/commit/34164140dd5aada52907a30d0ea483490d6da833))
    - handle entire crate deletions as well ([`eadc65f`](https://github.com/Byron/crates-index-diff-rs/commit/eadc65ff9b86cd02aeb29161511b6fec5d19cb04))
    - first sketch of addition is working ([`55d71dc`](https://github.com/Byron/crates-index-diff-rs/commit/55d71dc2f2f7c78af9940e1f9128e56753ac2191))
    - frame for diffing ([`cd86f5b`](https://github.com/Byron/crates-index-diff-rs/commit/cd86f5b82c8ac57106fa5ed1c254189fa894c3cd))
    - refactor ([`fa9cfab`](https://github.com/Byron/crates-index-diff-rs/commit/fa9cfab4053c630b8793a782ef732d7330b6f6c6))
    - Use `gitoxide` `Repository` instead of `git2::Repository` ([`2d3a182`](https://github.com/Byron/crates-index-diff-rs/commit/2d3a182819077a1fe068cb16fdfeceed2f6882da))
    - port all old tests to the new fixture ([`272bec8`](https://github.com/Byron/crates-index-diff-rs/commit/272bec8848e277eda4747523ed7497ef5d7f4d06))
    - test for auto-clone ([`8a1bc25`](https://github.com/Byron/crates-index-diff-rs/commit/8a1bc25ac020cc03513bf2bafd6d576b0dc2dded))
    - remove redundant tests ([`45494f0`](https://github.com/Byron/crates-index-diff-rs/commit/45494f081a2154f478929c19268f163e86595f29))
    - test for peek changes ([`61e217a`](https://github.com/Byron/crates-index-diff-rs/commit/61e217a2e0ef84e0f7bf091c0636a84804dd2fcf))
    - refactor ([`aeb6f45`](https://github.com/Byron/crates-index-diff-rs/commit/aeb6f45b9866e1c15862f336b6fe49bb3cf2dc2c))
    - use most recent git version of gitoxide for now ([`6dadfb7`](https://github.com/Byron/crates-index-diff-rs/commit/6dadfb759324e9858ea3a0774d6be89d6b9e5251))
    - thanks clippy ([`ebacafd`](https://github.com/Byron/crates-index-diff-rs/commit/ebacafd7855dda736fc5c6c90a608f06eb22b355))
    - normalization test ([`877b519`](https://github.com/Byron/crates-index-diff-rs/commit/877b5197fd13b7057b8daa6a75f9a517fa802d91))
    - add more tests for typical operations ([`56bfad7`](https://github.com/Byron/crates-index-diff-rs/commit/56bfad785be8dcc7259043b91cda8c4a267f617b))
    - first successful test for addition ([`365bcf0`](https://github.com/Byron/crates-index-diff-rs/commit/365bcf040b2493bb98da050fdcb6b420ac2f9b68))
    - simplify CI.yml ([`c0295c5`](https://github.com/Byron/crates-index-diff-rs/commit/c0295c53d8115e22536def570d9e09991bd186e9))
    - fix fixture script ([`0efccd4`](https://github.com/Byron/crates-index-diff-rs/commit/0efccd493aa61f4d33e1ffd5f18bb48d61555ce9))
    - first test can instantiate an `Index` on the new fixture ([`f9e31f2`](https://github.com/Byron/crates-index-diff-rs/commit/f9e31f20608ce093f590307d8fe46fd5fac91479))
    - add support for git-lfs to support archives ([`9a2ce43`](https://github.com/Byron/crates-index-diff-rs/commit/9a2ce43ef2961daef2951a0bc4fbc186917cd920))
    - build git repository from parts ([`d28591b`](https://github.com/Byron/crates-index-diff-rs/commit/d28591be86fb10b00f4db4f07cb1399c3b4305de))
    - also add commit-message information ([`7e85688`](https://github.com/Byron/crates-index-diff-rs/commit/7e85688272bc02e6e9ba923bfc72e219ee228a7a))
    - refactor ([`0c77e40`](https://github.com/Byron/crates-index-diff-rs/commit/0c77e40654abc741ae1f7ed08dba7129437a10bd))
    - refactor ([`78e05bd`](https://github.com/Byron/crates-index-diff-rs/commit/78e05bdd93ed3b88013ea5439b857d83f827e21c))
    - re-enable and fix doc-tests ([`946ca4c`](https://github.com/Byron/crates-index-diff-rs/commit/946ca4c8d7b9bc528569a89c6d2a14895c4e2314))
    - move `CloneOptions` into `index` module. ([`07f4b6c`](https://github.com/Byron/crates-index-diff-rs/commit/07f4b6c022ae8c48907250314db3a9eeb59ae89e))
    - refactor ([`ecd84eb`](https://github.com/Byron/crates-index-diff-rs/commit/ecd84eb489824abd7589526c864cbd8dfebb3a55))
    - a script to create an index fixture ([`9a5f312`](https://github.com/Byron/crates-index-diff-rs/commit/9a5f312b781e82a35d7ae9812e8d8095e371d656))
 * **Uncategorized**
    - Release crates-index-diff v11.0.0 ([`898024f`](https://github.com/Byron/crates-index-diff-rs/commit/898024ffe563e9b776f928fa9e41065ac2dcdd06))
    - Merge branch 'semantic-stability' ([`b7574d8`](https://github.com/Byron/crates-index-diff-rs/commit/b7574d8e518390e00d5eb50579c8644ed2f85eb2))
    - thanks clippy ([`9e9b972`](https://github.com/Byron/crates-index-diff-rs/commit/9e9b9726c4ea59ead04f071928042e65bc2e0204))
</details>

## 10.0.0 (2022-05-23)

### New Features (BREAKING)

 - <csr-id-38319375d07ca5d09700d40a778c367564cd8a66/> Add support for detecting deleted crates.
   Previously there was no need to do that as deletions couldn't happen -
   crates are yanked instead.
   
   Now that the ecosystem experienced its first (known) supply-chain attack
   crates can also be deleted and we should be able to detect that to allow
   downstream users to act on this automatically.

### Bug Fixes

 - <csr-id-d273245d99836ef799946373444a0b85e02523d0/> update version in usage example

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 84 calendar days.
 - 85 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v10.0.0 ([`43c63ca`](https://github.com/Byron/crates-index-diff-rs/commit/43c63ca51487aa15d6eac3ee5733809bdbaffd43))
    - prepare changelog prior to release ([`cc19788`](https://github.com/Byron/crates-index-diff-rs/commit/cc1978812ca6d6d0177fb3f2b4550181b5f32465))
    - Assure tests run serially without needing --jobs 1 ([`2701f5c`](https://github.com/Byron/crates-index-diff-rs/commit/2701f5c65fd104c9d481ebdf041806e6dee2f07a))
    - Merge branch 'syphar-handle-crate-delets' ([`aba9087`](https://github.com/Byron/crates-index-diff-rs/commit/aba908736924935c9d3b07ab793c28879368bc5f))
    - thanks clippy ([`df91215`](https://github.com/Byron/crates-index-diff-rs/commit/df912155a89f765853c6901e71df388558bd11b7))
    - disallow Rust 2018 idioms for clearer code ([`220b943`](https://github.com/Byron/crates-index-diff-rs/commit/220b9435b1b1da33410f4630166b41e376409df3))
    - Add support for detecting deleted crates. ([`3831937`](https://github.com/Byron/crates-index-diff-rs/commit/38319375d07ca5d09700d40a778c367564cd8a66))
    - Upgrade makefile for better auto-docs ([`0301da5`](https://github.com/Byron/crates-index-diff-rs/commit/0301da56a751018c8405ea0a46ba07487d9e2648))
    - refactor logic to handle crate-deletes ([`40655bd`](https://github.com/Byron/crates-index-diff-rs/commit/40655bdc5b1ba2ba20c6c9a99269fe13f124367a))
    - update version in usage example ([`d273245`](https://github.com/Byron/crates-index-diff-rs/commit/d273245d99836ef799946373444a0b85e02523d0))
</details>

## 9.0.0 (2022-02-26)

- Upgrade to `git2` v0.14, a BREAKING change. In order to use this release, assure that other dependencies also use `git2` v0.14.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 12 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v9.0.0 ([`31e82c7`](https://github.com/Byron/crates-index-diff-rs/commit/31e82c799c3da9dade6f78082aaf4640204eb44d))
    - update changelog; bump major version ([`997ea73`](https://github.com/Byron/crates-index-diff-rs/commit/997ea73473ffe42a7c72bf2bcfe93372902497da))
    - ignore certain tests fo now until new hashes are found that work ([`e78694f`](https://github.com/Byron/crates-index-diff-rs/commit/e78694f123f715bdd4bb79d150bcee8d00b49822))
    - Revert "see if this special case fixes tests" ([`5af8ec4`](https://github.com/Byron/crates-index-diff-rs/commit/5af8ec4b39ab067f73c0cb46526170270dbefefb))
    - prep for renaming 'master' to 'main' ([`e228d92`](https://github.com/Byron/crates-index-diff-rs/commit/e228d92e36869ae562ac5b272912ec773ce01d35))
    - see if this special case fixes tests ([`f00226b`](https://github.com/Byron/crates-index-diff-rs/commit/f00226be073cdab00afc7933eb7065a6c48ff972))
    - upgrade git2 to 0.14.0 ([`448eec7`](https://github.com/Byron/crates-index-diff-rs/commit/448eec7bb9cfeca8ea869429f3272aa44a750035))
</details>

## 8.0.1 (2022-02-14)

 - Only download the master branch on clone, not all branches, to greatly reduce the initial download size from nearly 800MB to just about 100MB.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 197 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release crates-index-diff v8.0.1 ([`91df107`](https://github.com/Byron/crates-index-diff-rs/commit/91df107100b41ae7448ae64976f9b404776382bc))
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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update to v2.0.0 ([`bd89e72`](https://github.com/Byron/crates-index-diff-rs/commit/bd89e7267b23d8a0bd801679d1ef74d12c84ba09))
    - use git2 version for compat with docs.rs ([`0ceebed`](https://github.com/Byron/crates-index-diff-rs/commit/0ceebed3d70c4482b5d09ffa1f9af5fea2bf7cd7))
</details>

## v1.0.1 (2016-12-26)

<csr-id-de4a284687fb476dd70bed3a4eb7e1737aff57ea/>
<csr-id-304dfafe95b23703f3b6d11230b487304d5d6bd0/>

### Other

 - <csr-id-304dfafe95b23703f3b6d11230b487304d5d6bd0/> crates.io badge
   [skip ci]

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
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v1.0.1 ([`de4a284`](https://github.com/Byron/crates-index-diff-rs/commit/de4a284687fb476dd70bed3a4eb7e1737aff57ea))
    - implementation for changetype ([`8ed9a81`](https://github.com/Byron/crates-index-diff-rs/commit/8ed9a81f0a84c43944f29f8407554303d84f7248))
    - make quick tests quick again ([`9aa756a`](https://github.com/Byron/crates-index-diff-rs/commit/9aa756ae534e78fc1c9148a0f6eda27ff07350b5))
    - crates.io badge ([`304dfaf`](https://github.com/Byron/crates-index-diff-rs/commit/304dfafe95b23703f3b6d11230b487304d5d6bd0))
</details>

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
<csr-id-706636b5198595ff8573505350f49aad241edfc6/>
<csr-id-91bf44d4f3c4454316f32489ba30cd250422065d/>
<csr-id-c4bf948b5e2c5590e58a134a3003acde7738e42d/>
<csr-id-b0f19b0a5d754cd9153b30ca9b363fa9534777da/>
<csr-id-56d416aae569d8dbcd568428a7489072eb749249/>
<csr-id-ed7ca366454a0c99698f18beb5955cd6606c7e1e/>
<csr-id-708d9c0680b797026da731bc9a9874ac71bc125b/>
<csr-id-2ef9c028812134af6bf23f72a4ea9850c407a06a/>
<csr-id-8048a2cf00618d669c9176b0e94353dd1cfa9011/>
<csr-id-887c088495ef78e21ca88091963dbfd0661e08ec/>
<csr-id-8801ec2d1d718eb73200d29ff23a958b5b1ba9d7/>
<csr-id-e451067a939a848082def317e1cceb487910aba2/>
<csr-id-d49f62fa41dbba9278ec2080ae2b91f72dc6834e/>
<csr-id-094c788f0b9ebd7beda17a8a7ee71d88ebbaad71/>
<csr-id-f9d531a63269e8e236489c9a7bb56a6bafdd9eeb/>

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

### Other

 - <csr-id-706636b5198595ff8573505350f49aad241edfc6/> docs for all remaining methods.
 - <csr-id-91bf44d4f3c4454316f32489ba30cd250422065d/> documentation for crateversion
 - <csr-id-c4bf948b5e2c5590e58a134a3003acde7738e42d/> customizations for us
   This could possibly work actually
 - <csr-id-b0f19b0a5d754cd9153b30ca9b363fa9534777da/> test osx as well
 - <csr-id-56d416aae569d8dbcd568428a7489072eb749249/> allow to change seen-ref name
 - <csr-id-ed7ca366454a0c99698f18beb5955cd6606c7e1e/> show backtrace
 - <csr-id-708d9c0680b797026da731bc9a9874ac71bc125b/> attempt of fetch_changes implementation
   It fails as it cannot create the correct reference.
 - <csr-id-2ef9c028812134af6bf23f72a4ea9850c407a06a/> support for unyanking
   We just count it as adding a crate, which also makes sense.
 - <csr-id-8048a2cf00618d669c9176b0e94353dd1cfa9011/> handle yanked files
 - <csr-id-887c088495ef78e21ca88091963dbfd0661e08ec/> now seeing the first added crates
 - <csr-id-8801ec2d1d718eb73200d29ff23a958b5b1ba9d7/> automate running tests quickly
   Using an existing checkout is important enough to put it into
   a makefile for documentation and automation.
 - <csr-id-e451067a939a848082def317e1cceb487910aba2/> support for commit'ishs for diffs
 - <csr-id-d49f62fa41dbba9278ec2080ae2b91f72dc6834e/> first traversal method
   The test fails for the wrong reason though, as it fails to
   parse my refspec even though libgit2 seems to be able to do it
   properly, and a recent-enough version is used.
 - <csr-id-094c788f0b9ebd7beda17a8a7ee71d88ebbaad71/> test against all versions of rust
 - <csr-id-f9d531a63269e8e236489c9a7bb56a6bafdd9eeb/> simplify travis
   travis-cargo does nothing for us in this case.

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
 - 61 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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

