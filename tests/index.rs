extern crate crates_index_diff;
extern crate tempdir;

use crates_index_diff::*;
use tempdir::TempDir;
use std::env;
use std::path::PathBuf;

const rev_one_added: &'static str = "6a65b384404d244cf3fa8571131a0187d8d5abc6";

#[test]
fn clone_if_needed() {
    let tmp = TempDir::new("new-index").unwrap();
    Index::at_path(tmp.path()).expect("successful clone to be created");
    Index::at_path(tmp.path()).expect("second instance re-uses existing clone");
}

#[test]
fn traverse_changed_crates() {
    let tmp = TempDir::new("new-index").unwrap();
    let index = Index::at_path(env::var("CRATES_INDEX_DIFF_TEST_EXISTING_INDEX")
            .map(PathBuf::from)
            .unwrap_or(tmp.path().to_owned()))
        .expect("successful clone");

    assert_eq!(index.traverse_changes("foo", rev_one_added).is_err(), true);
    assert_eq!(index.traverse_changes(rev_one_added, "bar").is_err(), true);

    let crates: Vec<Crate> = index.traverse_changes(format!("{}~1", rev_one_added), rev_one_added)
        .expect("ids to be valid and diff ok");
    assert_eq!(crates.len(), 1);
}
