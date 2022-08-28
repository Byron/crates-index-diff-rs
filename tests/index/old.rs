use crates_index_diff::*;
use git2::Reference;
use serial_test::serial;
use std::{env, path::PathBuf};
use tempdir::TempDir;

const NUM_VERSIONS_AT_RECENT_COMMIT: usize = 39752;
const REV_ONE_UNYANKED: &str = "f8cb00181";

#[test]
#[ignore] // This test takes too long for my taste, this library is stable by now
fn clone_if_needed() {
    let tmp = TempDir::new("new-index").unwrap();
    Index::from_path_or_cloned(tmp.path()).expect("successful clone to be created");
    Index::from_path_or_cloned(tmp.path()).expect("second instance re-uses existing clone");
}

fn make_index() -> (Index, TempDir) {
    let tmp = TempDir::new("new-index").unwrap();
    let index = Index::from_path_or_cloned(
        env::var("CRATES_INDEX_DIFF_TEST_EXISTING_INDEX")
            .map(PathBuf::from)
            .unwrap_or_else(|_| tmp.path().to_owned()),
    )
    .expect("successful clone");
    (index, tmp)
}

fn origin_master_of(index: &Index) -> Reference<'_> {
    index
        .repository()
        .find_reference("refs/remotes/origin/master")
        .unwrap()
}

#[test]
#[serial]
#[ignore]
fn quick_changes_since_last_fetch() {
    let (mut index, _tmp) = make_index();
    index.seen_ref_name = "refs/our-test-ref_because-we-can_hidden-from-ui";
    index
        .last_seen_reference()
        .and_then(|mut r| r.delete())
        .ok();
    let num_changes_since_first_commit = index.fetch_changes().unwrap().len();
    assert!(
        num_changes_since_first_commit >= NUM_VERSIONS_AT_RECENT_COMMIT,
        "should have fetched enough commits"
    );
    let mut seen_marker_ref = index
        .last_seen_reference()
        .expect("must be created/update now");
    assert!(
        seen_marker_ref == origin_master_of(&index),
        "should update the last_seen_reference to latest remote origin master"
    );

    // reset to previous one
    seen_marker_ref
        .set_target(
            index
                .repository()
                .revparse_single(REV_ONE_UNYANKED)
                .unwrap()
                .id(),
            "resetting to previous commit",
        )
        .expect("reset success");
    let num_seen_after_reset = index.fetch_changes().unwrap().len();
    let origin_master = origin_master_of(&index);
    assert!(
        seen_marker_ref == origin_master,
        "{} ({}) != {} ({})",
        seen_marker_ref.name().unwrap(),
        seen_marker_ref.peel_to_commit().unwrap().id(),
        origin_master.name().unwrap(),
        origin_master.peel_to_commit().unwrap().id()
    );
    assert!(num_seen_after_reset < num_changes_since_first_commit);
    assert!(num_seen_after_reset > 1000);

    // nothing if there was no change
    assert_eq!(index.fetch_changes().unwrap().len(), 0);
}
