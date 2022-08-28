use crates_index_diff::*;
use git2::Reference;
use serial_test::serial;
use std::{collections::HashMap, env, path::PathBuf};
use tempdir::TempDir;

const NUM_VERSIONS_AT_RECENT_COMMIT: usize = 39752;
// TODO: find new hashes for the ones below with similar states as they don't exist anymore. See ignored tests.
const REV_ONE_ADDED: &str = "615c9c41942a3ba13e088fbcb1470c61b169a187";
const REV_ONE_YANKED: &str = "8cf8fbad7876586ced34c4b778f6a80fadd2a59b";
const REV_ONE_UNYANKED: &str = "f8cb00181";
const REV_CRATE_DELETE: &str = "de5be3e8bb6cd7a3179857bdbdf28ca4fa23f84c";

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

fn changes_of(index: &Index, commit: &str) -> Vec<Change> {
    index
        .changes(format!("{}~1^{{tree}}", commit), commit)
        .expect("id to be valid and diff OK")
}

#[test]
#[serial]
#[ignore]
fn crate_delete() {
    let (index, _tmp) = make_index();

    let changes = changes_of(&index, REV_CRATE_DELETE);
    assert_eq!(changes, vec![Change::Deleted("rustdecimal".to_string())],);
}

#[test]
#[ignore]
#[serial]
fn quick_traverse_unyanked_crates() {
    //    [CrateVersion { dependencies: [Dependency { name: "freetype-rs", required_version: "^0.11", features: [], optional: false, default_features: true, target: None, kind: Some("normal"), package: None }, Dependency { name: "gfx", required_version: "^0.12.2", features: [], optional: false, default_features: true, target: None, kind: Some("normal"), package: None }, Dependency { name: "glutin", required_version: "^0.6", features: [], optional: false, default_features: true, target: None, kind: Some("dev"), package: None }, Dependency { name: "gfx_window_glutin", required_version: "^0.12", features: [], optional: false, default_features: true, target: None, kind: Some("dev"), package: None }] }]
    let (index, _tmp) = make_index();

    let crates = changes_of(&index, REV_ONE_UNYANKED);
    assert_eq!(
        crates,
        vec![Change::Added(CrateVersion {
            name: "gfx_text".to_owned(),
            yanked: false,
            version: "0.13.2".to_owned(),
            dependencies: vec![
                Dependency {
                    name: "freetype-rs".into(),
                    required_version: "^0.11".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("normal".into()),
                    package: None
                },
                Dependency {
                    name: "gfx".into(),
                    required_version: "^0.12.2".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("normal".into()),
                    package: None
                },
                Dependency {
                    name: "glutin".into(),
                    required_version: "^0.6".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("dev".into()),
                    package: None
                },
                Dependency {
                    name: "gfx_window_glutin".into(),
                    required_version: "^0.12".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("dev".into()),
                    package: None
                }
            ],
            features: {
                let mut h = HashMap::new();
                h.insert("default".to_string(), vec!["include-font".to_string()]);
                h.insert("include-font".into(), vec![]);
                h
            },
            checksum: "d0b1240e3627e646f69685ddd3e7d83dd3ff3d586afe83bf3679082028183f2d".into(),
        })]
    );
}

#[test]
#[ignore]
#[serial]
fn quick_traverse_yanked_crates() {
    let (index, _tmp) = make_index();

    let crates = changes_of(&index, REV_ONE_YANKED);
    assert_eq!(
        crates,
        vec![Change::Yanked(CrateVersion {
            name: "sha3".to_owned(),
            yanked: true,
            version: "0.0.0".to_owned(),
            dependencies: Vec::new(),
            features: HashMap::new(),
            checksum: "dbba9d72d3d04e2167fb9c76ce22aed118eb003727bbe59774b9bf3603fa1f43".into(),
        })]
    );
}

#[test]
#[ignore]
#[serial]
fn quick_traverse_added_crates() {
    let (index, _tmp) = make_index();
    assert!(index.changes("foo", REV_ONE_ADDED).is_err());
    assert!(index.changes(REV_ONE_ADDED, "bar").is_err());

    let crates = changes_of(&index, REV_ONE_ADDED);
    assert_eq!(
        crates,
        vec![Change::Added(CrateVersion {
            name: "rpwg".to_owned(),
            yanked: false,
            version: "0.1.0".to_owned(),
            dependencies: vec![
                Dependency {
                    name: "rand".into(),
                    required_version: "^0.3".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("normal".into()),
                    package: None
                },
                Dependency {
                    name: "clap".into(),
                    required_version: "^2.19".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("normal".into()),
                    package: None
                }
            ],
            features: HashMap::new(),
            checksum: "14437a3702699dba0c49ddc401a0529898e83f8b769348549985a0f4d818d3ca".into(),
        })]
    );
}
