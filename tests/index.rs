use crates_index_diff::*;
use git2::Reference;
use std::{collections::HashMap, env, path::PathBuf};
use tempdir::TempDir;

const NUM_VERSIONS_AT_RECENT_COMMIT: usize = 39752;
const REV_ONE_ADDED: &'static str = "615c9c41942a3ba13e088fbcb1470c61b169a187";
const REV_ONE_YANKED: &'static str = "8cf8fbad7876586ced34c4b778f6a80fadd2a59b";
const REV_ONE_UNYANKED: &'static str = "f8cb00181";

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
            .unwrap_or(tmp.path().to_owned()),
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
    assert!(seen_marker_ref == origin_master_of(&index));
    assert!(num_seen_after_reset < num_changes_since_first_commit);
    assert!(num_seen_after_reset > 1000);

    // nothing if there was no change
    assert_eq!(index.fetch_changes().unwrap().len(), 0);
}

#[test]
fn peek_changes_since_last_fetch() {
    let (mut index, _tmp) = make_index();
    index.seen_ref_name = "refs/our-test-ref_because-we-can_hidden-from-ui";
    index
        .last_seen_reference()
        .and_then(|mut r| r.delete())
        .ok();
    let (changes, last_seen_rev) = index.peek_changes().unwrap();
    assert!(changes.len() >= NUM_VERSIONS_AT_RECENT_COMMIT);
    assert_eq!(
        last_seen_rev,
        origin_master_of(&index).target().unwrap(),
        "last seen reference should be origin"
    );
    assert!(
        index.last_seen_reference().is_err(),
        "the last-seen reference has not been created (or updated, but we don't test that yet)"
    );
}

fn changes_of(index: &Index, commit: &str) -> Vec<CrateVersion> {
    index
        .changes(format!("{}~1^{{tree}}", commit), format!("{}", commit))
        .expect("id to be valid and diff OK")
}

#[test]
fn quick_traverse_unyanked_crates() {
    //    [CrateVersion { dependencies: [Dependency { name: "freetype-rs", required_version: "^0.11", features: [], optional: false, default_features: true, target: None, kind: Some("normal"), package: None }, Dependency { name: "gfx", required_version: "^0.12.2", features: [], optional: false, default_features: true, target: None, kind: Some("normal"), package: None }, Dependency { name: "glutin", required_version: "^0.6", features: [], optional: false, default_features: true, target: None, kind: Some("dev"), package: None }, Dependency { name: "gfx_window_glutin", required_version: "^0.12", features: [], optional: false, default_features: true, target: None, kind: Some("dev"), package: None }] }]
    let (index, _tmp) = make_index();

    let crates = changes_of(&index, REV_ONE_UNYANKED);
    assert_eq!(
        crates,
        vec![CrateVersion {
            name: "gfx_text".to_owned(),
            kind: ChangeKind::Added,
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
        }]
    );
}

#[test]
fn quick_traverse_yanked_crates() {
    let (index, _tmp) = make_index();

    let crates = changes_of(&index, REV_ONE_YANKED);
    assert_eq!(
        crates,
        vec![CrateVersion {
            name: "sha3".to_owned(),
            kind: ChangeKind::Yanked,
            version: "0.0.0".to_owned(),
            dependencies: Vec::new(),
            features: HashMap::new(),
            checksum: "dbba9d72d3d04e2167fb9c76ce22aed118eb003727bbe59774b9bf3603fa1f43".into(),
        }]
    );
}

#[test]
fn quick_traverse_added_crates() {
    let (index, _tmp) = make_index();
    assert_eq!(index.changes("foo", REV_ONE_ADDED).is_err(), true);
    assert_eq!(index.changes(REV_ONE_ADDED, "bar").is_err(), true);

    let crates = changes_of(&index, REV_ONE_ADDED);
    assert_eq!(
        crates,
        vec![CrateVersion {
            name: "rpwg".to_owned(),
            kind: ChangeKind::Added,
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
        }]
    );
}
