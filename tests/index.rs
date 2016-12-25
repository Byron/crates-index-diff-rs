extern crate crates_index_diff;
extern crate tempdir;

use crates_index_diff::*;
use tempdir::TempDir;
use std::env;
use std::path::PathBuf;

const rev_one_added: &'static str = "615c9c41942a3ba13e088fbcb1470c61b169a187";

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

    let crates: Vec<Crate> = index.traverse_changes(format!("{}~1^{{tree}}", rev_one_added), format!("{}^{{tree}}", rev_one_added))
        .expect("ids to be valid and diff ok");
    assert_eq!(crates.len(), 1);
}
