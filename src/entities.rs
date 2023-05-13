use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use tracing::error;

pub type EntityMapType = HashMap<String, String>;

#[tracing::instrument]
pub fn generate_entity_map(doc_type_decl: String) -> EntityMapType {
    lazy_static! {
        static ref REGEX_GET_ENTITY_MAP: Regex =
            Regex::new(r#"(?m)^<!ENTITY\s(?P<key>[\w-]+)\s?"(?P<value>(.+))">$"#).unwrap();
    };
    let mut entity_map: EntityMapType = HashMap::new();
    let test = REGEX_GET_ENTITY_MAP.captures_iter(doc_type_decl.as_str());
    for i in test {
        let value: String = i
            .name("value")
            .map_or(String::default(), |f| f.as_str().to_string());
        let key: String = i
            .name("key")
            .map_or(String::default(), |f| f.as_str().to_string());
        if !key.is_empty() {
            entity_map.insert(key, value);
        } else {
            error!({key=?i.name("key")}, "MISSING KEY!");
        }
    }
    entity_map
}
#[cfg(test)]
mod tests {

    use crate::entities::generate_entity_map;

    static DOC_TYPE: &str = include_str!("../tests/jmdict_doctype.txt");
    #[test]
    fn ensure_doc_type_entities_read() {
        //let parser = JMDictParser::new_from_string(XML_FILE.to_string(), None);
        //let dict = parser.parse();
        //let dict = dict.as_ref().borrow();
        //if let ElementDataVariant::JMDict(JMElementData::Document(x)) = &dict.object {
        //    println!("HAHA!");
        //    assert!(!x.entities.is_empty())
        //}
        let entities = generate_entity_map(DOC_TYPE.to_string().replace("\r\n", "\n"));
        assert!(!entities.is_empty())
    }
}

