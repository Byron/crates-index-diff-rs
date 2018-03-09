extern crate crates_index_diff;
extern crate serde_json;

use crates_index_diff::*;

#[test]
fn test_parse_crate_version() {
    let c: CrateVersion = serde_json::from_str(
        r#"{
        "name": "test",
        "vers": "1.0.0",
        "yanked": true
    }"#).unwrap();
    assert_eq!(
        c,
        CrateVersion {
            name: "test".to_string(),
            kind: ChangeKind::Yanked,
            version: "1.0.0".to_string(),
        }
    );
}
