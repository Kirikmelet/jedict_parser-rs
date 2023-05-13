/* Module for Dictionary Sanitation */

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Serialize, Deserialize};

use crate::{elements::JElementDataVariant, entities::EntityMapType};

pub type DictResult<T> = Result<T, Box<DictError>>;

#[derive(Debug, Clone)]
pub enum DictErrorKind {
    Generic,
    WrongType(JElementDataVariant), // Expected Type
}
#[derive(Debug, Clone)]
pub struct DictError {
    pub text: String,
    pub dom_type: JElementDataVariant,
    pub kind: DictErrorKind,
}

impl DictError {
    pub fn throw_wrong_type(
        dom_type: JElementDataVariant,
        expected_type: JElementDataVariant,
    ) -> Box<Self> {
        Box::new(DictError {
            text: String::new(),
            dom_type,
            kind: DictErrorKind::WrongType(expected_type),
        })
    }
    pub fn throw_generic(text: String) -> Box<Self> {
        Box::new(DictError {
            text,
            dom_type: JElementDataVariant::Nil,
            kind: DictErrorKind::Generic,
        })
    }
}

impl std::fmt::Display for DictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DictErrorKind::WrongType(x) => {
                write!(f, "Wrong Type!: {}\nExpected Type!{}", self.dom_type, x)
            }
            _ => {
                write!(f, "Generic Error: {} | {}", self.text, self.dom_type)
            }
        }
    }
}

#[tracing::instrument]
pub fn get_entity(data: String, entities: &EntityMapType) -> String {
    lazy_static! {
        static ref REGEX_FIND_ENTITY: Regex = Regex::new("&([a-z]*);").unwrap();
    };
    if let Some(capture) = REGEX_FIND_ENTITY.captures(&data) {
        let Some(data) = capture.get(1) else {
                return data
            };
        let data = data.as_str().to_string();
        return match entities.get(&data) {
            Some(x) => x.to_string(),
            None => data,
        };
    };
    data
}

pub trait DictToJson {
    fn to_json<'a, T>(&self) -> String where Self: Deserialize<'a> + Serialize {
        serde_json::to_string(self).unwrap_or(String::default())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::dict;
    use crate::entities::EntityMapType;

    #[test]
    fn ensure_entity_capture() {
        let mut entity_map: EntityMapType = HashMap::new();
        entity_map.insert(String::from("test"), String::from("Test Module"));
        let assertion = dict::get_entity(String::from("&test;"), &entity_map);
        assert!(assertion.eq("Test Module"))
    }
}
