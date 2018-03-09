use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use std::fmt;

/// Identify a kind of change that occurred to a crate
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ChangeKind {
    /// A crate version was added
    Added,
    /// A crate version was added or it was unyanked.
    Yanked,
}

impl<'de> Deserialize<'de> for ChangeKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct Visitor;
        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ChangeKind;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("boolean")
            }
            fn visit_bool<E>(self, value: bool) -> Result<ChangeKind, E>
                where E: ::serde::de::Error
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
        where S: Serializer
    {
        serializer.serialize_bool(self == &ChangeKind::Yanked)
    }
}

impl fmt::Display for ChangeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   ChangeKind::Added => "added",
                   ChangeKind::Yanked => "yanked",
               })
    }
}

/// Pack all information we know about a change made to a version of a crate.
#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct CrateVersion {
    /// The crate name, i.e. `clap`.
    pub name: String,
    /// The kind of change.
    #[serde(rename="yanked")]
    pub kind: ChangeKind,
    /// The semantic version of the crate.
    #[serde(rename="vers")]
    pub version: String,
}
