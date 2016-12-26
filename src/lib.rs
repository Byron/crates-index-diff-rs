//! Learn what's changed in the crates index.
//!
//! TODO: example usage
#[macro_use] extern crate quick_error;
extern crate git2;
extern crate rustc_serialize;

mod crate_;
mod index;

pub use crate_::*;
pub use index::*;
