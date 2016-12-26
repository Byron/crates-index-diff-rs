use rustc_serialize::json::{self, Json};

use std::fmt::{self, Display};

/// Identify a kind of change that occurred to a crate
#[derive(RustcEncodable, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ChangeKind {
    /// A crate version was added
    Added,
    /// A crate version was added or it was unyanked.
    Yanked,
}

impl Display for ChangeKind {
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
#[derive(RustcEncodable, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct CrateVersion {
    /// The crate name, i.e. `clap`.
    pub name: String,
    /// The kind of change.
    pub kind: ChangeKind,
    /// The semantic version of the crate.
    pub version: String,
}

quick_error! {
    /// Error type to identify problems when decoding a crate version.
    #[derive(PartialOrd, PartialEq, Debug)]
    pub enum CrateVersionDecodeError {
        /// A field is missing in an object
        MissingFieldError {
            object: json::Object,
            field: &'static str,
        } {
            description("A field is not present in a json object")
            display("Field '{}' was missing in object '{:?}'", field, object)
        }
        /// An object was expected, but not obtained
        ObjectExpected { json: Json } {
            description("A json object was expected")
            display("Json was not an object: '{:?}'", json)
        }
        /// A string was expected, but not obtained
        StringExpected { json: Json } {
            description("A json string was expected")
            display("Json was not an string: '{:?}'", json)
        }
        /// A boolean was expected, but not obtained
        BoolExpected { json: Json } {
            description("A json boolean was expected")
            display("Json was not an boolean: '{:?}'", json)
        }
    }
}

use self::CrateVersionDecodeError::*;

impl CrateVersion {
    /// Return a new version as decoded from a Json from a diff of the crates.io-index.
    pub fn from_crates_diff_json(value: Json) -> Result<CrateVersion, CrateVersionDecodeError> {
        fn extract<'a>(o: &'a json::Object,
                       field: &'static str)
                       -> Result<&'a Json, CrateVersionDecodeError> {
            o.get(field).ok_or_else(|| {
                MissingFieldError {
                    object: o.clone(),
                    field: field,
                }
            })
        }

        fn into_string(value: &Json) -> Result<String, CrateVersionDecodeError> {
            value.as_string()
                .ok_or_else(|| StringExpected { json: value.clone() })
                .map(Into::into)
        }

        fn into_bool(value: &Json) -> Result<bool, CrateVersionDecodeError> {
            value.as_boolean()
                .ok_or_else(|| BoolExpected { json: value.clone() })
                .map(Into::into)
        }

        let o = value.as_object().ok_or_else(|| ObjectExpected { json: value.clone() })?;
        let name = extract(o, "name").and_then(into_string)?;
        let version = extract(o, "vers").and_then(into_string)?;
        let yanked = extract(o, "yanked").and_then(into_bool)?;

        Ok(CrateVersion {
            name: name,
            kind: if yanked {
                ChangeKind::Yanked
            } else {
                ChangeKind::Added
            },
            version: version,
        })
    }
}
