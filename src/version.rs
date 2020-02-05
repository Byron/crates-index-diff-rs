use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;

use std::fmt;

/// Identify a kind of change that occurred to a crate
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ChangeKind {
    /// A crate version was added
    Added,
    /// A crate version was added or it was unyanked.
    Yanked,
}

impl Default for ChangeKind {
    fn default() -> Self {
        ChangeKind::Added
    }
}

impl<'de> Deserialize<'de> for ChangeKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ChangeKind;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("boolean")
            }
            fn visit_bool<E>(self, value: bool) -> Result<ChangeKind, E>
            where
                E: ::serde::de::Error,
            {
                if value {
                    Ok(ChangeKind::Yanked)
                } else {
                    Ok(ChangeKind::Added)
                }
            }
        }
        deserializer.deserialize_bool(Visitor)
    }
}

impl Serialize for ChangeKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self == &ChangeKind::Yanked)
    }
}

impl fmt::Display for ChangeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ChangeKind::Added => "added",
                ChangeKind::Yanked => "yanked",
            }
        )
    }
}

/// Pack all information we know about a change made to a version of a crate.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct CrateVersion {
    /// The crate name, i.e. `clap`.
    pub name: String,
    /// The kind of change.
    #[serde(rename = "yanked")]
    pub kind: ChangeKind,
    /// The semantic version of the crate.
    #[serde(rename = "vers")]
    pub version: String,
    /// The checksum over the crate archive
    #[serde(rename = "cksum")]
    pub checksum: String,
    /// All cargo features
    pub features: HashMap<String, Vec<String>>,
    /// All crate dependencies
    #[serde(rename = "deps")]
    pub dependencies: Vec<Dependency>,
}

/// A single dependency of a specific crate version
#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Dependency {
    /// The crate name
    pub name: String,
    /// The version the parent crate requires of this dependency
    #[serde(rename = "req")]
    pub required_version: String,
    /// All cargo features configured by the parent crate
    pub features: Vec<String>,
    /// True if this is an optional dependency
    pub optional: bool,
    /// True if default features are enabled
    pub default_features: bool,
    /// The name of the build target
    pub target: Option<String>,
    /// The kind of dependency, usually 'normal' or 'dev'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The package this crate is contained in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
}
