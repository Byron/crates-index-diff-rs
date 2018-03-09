//! Learn what's changed in the crates index.
//!
//! Have a look at the real-world usage to learn more about it:
//! [crates-io-cli](https://github.com/Byron/crates-io-cli-rs/blob/b7a39ad8ef68adb81b2d8a7e552cb0a2a73f7d5b/src/main.rs#L62)
extern crate git2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod version;
mod index;

pub use version::*;
pub use index::*;
