extern crate crates_index_diff;
extern crate tempdir;

use crates_index_diff::*;
use tempdir::TempDir;
use std::env;
use std::path::PathBuf;

const REV_ONE_ADDED: &'static str = "615c9c41942a3ba13e088fbcb1470c61b169a187";
const REV_ONE_YANKED: &'static str = "8cf8fbad7876586ced34c4b778f6a80fadd2a59b";
const REV_ONE_UNYANKED: &'static str = "f8cb00181";

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
fn quick_traverse_added_crate() {
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
