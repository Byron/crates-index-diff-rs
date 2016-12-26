use rustc_serialize::json::{self, Json};

#[derive(RustcEncodable, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ChangeType {
    Added,
    Yanked,
}

#[derive(RustcEncodable, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Crate {
    pub name: String,
    pub state: ChangeType,
    pub version: String,
}

quick_error! {
    #[derive(PartialOrd, PartialEq, Debug)]
    pub enum CrateDecodeError {
        MissingFieldError {
            object: json::Object,
            field: &'static str,
        } {
            description("A field is not present in a json object")
            display("Field '{}' was missing in object '{:?}'", field, object)
        }
        ObjectExpected { json: Json } {
            description("A json object was expected")
            display("Json was not an object: '{:?}'", json)
        }
        StringExpected { json: Json } {
            description("A json string was expected")
            display("Json was not an string: '{:?}'", json)
        }
        BoolExpected { json: Json } {
            description("A json boolean was expected")
            display("Json was not an boolean: '{:?}'", json)
        }
    }
}

use self::CrateDecodeError::*;

impl Crate {
    pub fn from_crates_diff_json(value: Json) -> Result<Crate, CrateDecodeError> {
        fn extract<'a>(o: &'a json::Object,
                       field: &'static str)
                       -> Result<&'a Json, CrateDecodeError> {
            o.get(field).ok_or_else(|| {
                MissingFieldError {
                    object: o.clone(),
                    field: field,
                }
            })
        }

        fn into_string(value: &Json) -> Result<String, CrateDecodeError> {
            value.as_string()
                 .ok_or_else(|| StringExpected { json: value.clone() })
                 .map(Into::into)
        }

        fn into_bool(value: &Json) -> Result<bool, CrateDecodeError> {
            value.as_boolean()
                 .ok_or_else(|| BoolExpected { json: value.clone() })
                 .map(Into::into)
        }

        let o = value.as_object().ok_or_else(|| ObjectExpected { json: value.clone() })?;
        let name = extract(o, "name").and_then(into_string)?;
        let version = extract(o, "vers").and_then(into_string)?;
        let yanked = extract(o, "yanked").and_then(into_bool)?;

        Ok(Crate {
            name: name,
            state: if yanked {
                ChangeType::Yanked
            } else {
                ChangeType::Added
            },
            version: version,
        })
    }
}
