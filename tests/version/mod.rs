use crates_index_diff::*;
use std::collections::HashMap;

#[test]
fn parse_crate_version() {
    let c: CrateVersion = serde_json::from_str(
        r#"{
        "name": "test",
        "vers": "1.0.0",
        "cksum": "cksum",
        "features" : {},
        "deps" : [],
        "yanked": true
    }"#,
    )
    .unwrap();
    assert_eq!(
        c,
        CrateVersion {
            name: "test".to_string(),
            yanked: true,
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
            features: HashMap::new(),
            checksum: "cksum".into()
        }
    );
}
