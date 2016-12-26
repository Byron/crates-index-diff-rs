//! Learn what's changed in the crates index.
//!
#[macro_use]
extern crate quick_error;
extern crate git2;
extern crate rustc_serialize;

mod version;
mod index;

pub use version::*;
pub use index::*;
