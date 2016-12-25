extern crate crates_index_diff;
extern crate tempdir;

use crates_index_diff::*;
use tempdir::TempDir;

#[test]
fn clone_if_needed() {
    let tmp: TempDir = TempDir::new("new-index").unwrap();
    Index::at_path(tmp.path()).expect("successful clone to be created");
    Index::at_path(tmp.path()).expect("second instance re-uses existing clone");
}
