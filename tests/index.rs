extern crate crates_index_diff;
extern crate tempdir;

use crates_index_diff::*;
use tempdir::TempDir;
use std::env;
use std::path::PathBuf;

const NUM_VERSIONS_AT_RECENT_COMMIT: usize = 39752;
const REV_ONE_ADDED: &'static str = "615c9c41942a3ba13e088fbcb1470c61b169a187";
const REV_ONE_YANKED: &'static str = "8cf8fbad7876586ced34c4b778f6a80fadd2a59b";
const REV_ONE_UNYANKED: &'static str = "f8cb00181";

#[test]
fn clone_if_needed() {
    let tmp = TempDir::new("new-index").unwrap();
    Index::from_path_or_cloned(tmp.path()).expect("successful clone to be created");
    Index::from_path_or_cloned(tmp.path()).expect("second instance re-uses existing clone");
}

fn make_index() -> (Index, TempDir) {
    let tmp = TempDir::new("new-index").unwrap();
    let index = Index::from_path_or_cloned(env::var("CRATES_INDEX_DIFF_TEST_EXISTING_INDEX")
            .map(PathBuf::from)
            .unwrap_or(tmp.path().to_owned()))
        .expect("successful clone");
    (index, tmp)
}

#[test]
fn quick_changes_since_last_fetch() {
    let (mut index, _) = make_index();
    index.seen_ref_name = "refs/our-test-ref_because-we-can_hidden-from-ui";
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
    assert!(num_seen_after_reset > 1000);

    // nothing if there was no change
    assert_eq!(index.fetch_changes().unwrap().len(), 0);
}

fn changes_of(index: &Index, commit: &str) -> Vec<CrateVersion> {
    index.changes(format!("{}~1^{{tree}}", commit), format!("{}", commit))
        .expect("id to be valid and diff OK")
}


#[test]
fn quick_traverse_unyanked_crates() {
    let (index, _) = make_index();

    let crates = changes_of(&index, REV_ONE_UNYANKED);
    assert_eq!(crates,
               vec![CrateVersion {
                        name: "gfx_text".to_owned(),
                        kind: ChangeKind::Added,
                        version: "0.13.2".to_owned(),
                    }]);
}

#[test]
fn quick_traverse_yanked_crates() {
    let (index, _) = make_index();

    let crates = changes_of(&index, REV_ONE_YANKED);
    assert_eq!(crates,
               vec![CrateVersion {
                        name: "sha3".to_owned(),
                        kind: ChangeKind::Yanked,
                        version: "0.0.0".to_owned(),
                    }]);
}

#[test]
fn quick_traverse_added_crates() {
    let (index, _) = make_index();
    assert_eq!(index.changes("foo", REV_ONE_ADDED).is_err(), true);
    assert_eq!(index.changes(REV_ONE_ADDED, "bar").is_err(), true);

    let crates = changes_of(&index, REV_ONE_ADDED);
    assert_eq!(crates,
               vec![CrateVersion {
                        name: "rpwg".to_owned(),
                        kind: ChangeKind::Added,
                        version: "0.1.0".to_owned(),
                    }]);
}
