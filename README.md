[![Build Status](https://travis-ci.org/Byron/crates-index-diff-rs.svg?branch=master)](https://travis-ci.org/Byron/crates-index-diff-rs)

A library to easily retrieve changes between different revisions of the crates.io index.

It will only need a bare clone, which saves resources.

# Usage

Add this to your Cargo.toml
```toml
[dependencies]
crates-index-diff = "*"
```

Add this to your lib ...
```Rust
extern crate crates_index_diff;
```

