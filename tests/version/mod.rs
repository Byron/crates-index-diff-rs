use crates_index_diff::*;
use semver::{Version, VersionReq};
use serde_json::json;
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
    assert_eq!(c.version(), Version::new(1, 0, 0));
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

#[test]
fn parse_dependency() {
    let d: Dependency = serde_json::from_value(json!({
        "name": "dep_crate",
        "req": "^1.2.3",
        "features": ["feature1", "feature2"],
        "optional": false,
        "default_features": true,
    }))
    .unwrap();
    assert_eq!(d.required_version(), VersionReq::parse("^1.2.3").unwrap());
    assert_eq!(
        d,
        Dependency {
            name: "dep_crate".into(),
            required_version: "^1.2.3".into(),
            features: vec!["feature1".into(), "feature2".into()],
            optional: false,
            default_features: true,
            target: None,
            kind: None,
            package: None
        }
    );
}

#[test]
fn parse_crate_version_with_dependencies() {
    let c: CrateVersion = serde_json::from_value(json!({
        "name": "test",
        "vers": "1.0.0",
        "cksum": "0000000000000000000000000000000000000000000000000000000000000000",
        "features" : {},
        "deps" : [
            {
                "name": "dep_crate",
                "req": ">=1.2.3, <1.8.0",
                "features": ["feature1", "feature2"],
                "optional": false,
                "default_features": true,
                "target": "main",
                "kind": "dev",
                "package": "dep_package"
            }
        ],
        "yanked": true
    }))
    .unwrap();
    assert_eq!(c.version(), Version::new(1, 0, 0));
    assert_eq!(
        c.dependencies[0].required_version(),
        VersionReq::parse(">=1.2.3, <1.8.0").unwrap()
    );
    assert_eq!(
        c,
        CrateVersion {
            name: "test".into(),
            yanked: true,
            version: "1.0.0".into(),
            dependencies: vec![Dependency {
                name: "dep_crate".into(),
                required_version: ">=1.2.3, <1.8.0".into(),
                features: vec!["feature1".into(), "feature2".into()],
                optional: false,
                default_features: true,
                target: Some("main".into()),
                kind: Some(DependencyKind::Dev),
                package: Some("dep_package".into())
            }],
            features: HashMap::new(),
            checksum: Default::default()
        }
    );
}
