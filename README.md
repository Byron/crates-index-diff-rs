[![Build Status linux+osx](https://travis-ci.org/Byron/crates-index-diff-rs.svg?branch=master)](https://travis-ci.org/Byron/crates-index-diff-rs)
[![Build status windows](https://ci.appveyor.com/api/projects/status/y7kfoniyl1uuxek3/branch/master?svg=true&passingText=windows%20OK&failingText=windows%20failed)](https://ci.appveyor.com/project/Byron/crates-index-diff-rs/branch/master)


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

