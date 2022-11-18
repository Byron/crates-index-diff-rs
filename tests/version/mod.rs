use crates_index_diff::*;
use std::collections::HashMap;

#[test]
fn parse_crate_version() {
    let c: CrateVersion = serde_json::from_str(
        r#"{
        "name": "test",
        "vers": "1.0.0",
        "cksum": "0000000000000000000000000000000000000000000000000000000000000000",
        "features" : {},
        "deps" : [],
        "yanked": true
    }"#,
    )
    .unwrap();
    assert_eq!(
        c,
        CrateVersion {
            name: "test".into(),
            yanked: true,
            version: "1.0.0".into(),
            dependencies: Vec::new(),
            features: HashMap::new(),
            checksum: Default::default()
        }
    );
}
