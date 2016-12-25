extern crate crates_index_diff;
extern crate tempdir;

use crates_index_diff::*;
use tempdir::TempDir;
use std::env;
use std::path::PathBuf;

const NUM_VERSIONS_AT_RECENT_COMMIT: usize = 39752;
const REV_ONE_ADDED: &'static str = "615c9c41942a3ba13e088fbcb1470c61b169a187";
const REV_RECENT_COMMIT: &'static str = REV_ONE_ADDED;
const REV_ONE_YANKED: &'static str = "8cf8fbad7876586ced34c4b778f6a80fadd2a59b";
const REV_ONE_UNYANKED: &'static str = "f8cb00181";
const REV_FIRST_COMMIT: &'static str = "a33de1c98";

#[test]
fn clone_if_needed() {
    let tmp = TempDir::new("new-index").unwrap();
    Index::at_path(tmp.path()).expect("successful clone to be created");
    Index::at_path(tmp.path()).expect("second instance re-uses existing clone");
}

fn make_index() -> (Index, TempDir) {
    let tmp = TempDir::new("new-index").unwrap();
    let index = Index::at_path(env::var("CRATES_INDEX_DIFF_TEST_EXISTING_INDEX")
            .map(PathBuf::from)
            .unwrap_or(tmp.path().to_owned()))
        .expect("successful clone");
    (index, tmp)
}

#[test]
fn quick_changes_since_last_fetch() {
    let (index, _) = make_index();
    let origin_master = || index.repository().find_reference("refs/remotes/origin/master").unwrap();
    index.last_seen_reference().and_then(|mut r| r.delete()).ok();
    let num_changes_since_first_commit = index.fetch_changes().unwrap().len();
    assert!(num_changes_since_first_commit >= NUM_VERSIONS_AT_RECENT_COMMIT);
    let mut seen_marker_ref = index.last_seen_reference().expect("must be created/update now");
    assert!(seen_marker_ref == origin_master());

    // reset to previous one
    seen_marker_ref.set_target(index.repository().revparse_single(REV_ONE_UNYANKED).unwrap().id(),
                    "resetting to previous commit")
        .expect("reset success");
    let num_seen_after_reset = index.fetch_changes().unwrap().len();
    assert!(seen_marker_ref == origin_master());
    assert!(num_seen_after_reset < num_changes_since_first_commit);
    assert!(num_seen_after_reset < NUM_VERSIONS_AT_RECENT_COMMIT);
    assert!(num_seen_after_reset > 1000);

    // nothing if there was no change
    assert_eq!(index.fetch_changes().unwrap().len(), 0);
}

fn changes_of(index: &Index, commit: &str) -> Vec<Crate> {
    index.changes(format!("{}~1^{{tree}}", commit), format!("{}", commit))
        .expect("id to be valid and diff OK")
}


#[test]
fn quick_traverse_unyanked_crates() {
    let (index, _) = make_index();

    let crates = changes_of(&index, REV_ONE_UNYANKED);
    assert_eq!(crates,
               vec![Crate {
                        name: "gfx_text".to_owned(),
                        state: ChangeType::Added,
                        version: "0.13.2".to_owned(),
                    }]);
}

#[test]
fn quick_traverse_yanked_crates() {
    let (index, _) = make_index();

    let crates = changes_of(&index, REV_ONE_YANKED);
    assert_eq!(crates,
               vec![Crate {
                        name: "sha3".to_owned(),
                        state: ChangeType::Yanked,
                        version: "0.0.0".to_owned(),
                    }]);
}

#[test]
fn quick_traverse_all_crates() {
    let (index, _) = make_index();
    let changes = index.changes(format!("{}", REV_FIRST_COMMIT),
                 format!("{}", REV_RECENT_COMMIT))
        .expect("id to be valid and diff OK");
    assert_eq!(changes.len(), NUM_VERSIONS_AT_RECENT_COMMIT);
}

#[test]
fn quick_traverse_added_crates() {
    let (index, _) = make_index();
    assert_eq!(index.changes("foo", REV_ONE_ADDED).is_err(), true);
    assert_eq!(index.changes(REV_ONE_ADDED, "bar").is_err(), true);

    let crates = changes_of(&index, REV_ONE_ADDED);
    assert_eq!(crates,
               vec![Crate {
                        name: "rpwg".to_owned(),
                        state: ChangeType::Added,
                        version: "0.1.0".to_owned(),
                    }]);
}
