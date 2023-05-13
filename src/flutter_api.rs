use crate::kanjidic2::dict::{KD2DictCharacter, KD2DictHeader};
pub use crate::{
    dict::DictToJson, jmdict::dict::JMDict, jndict::dict::JNDict, parser::jedict_parser_configs,
};
pub use crate::{
    jmdict::dict::{JMDictDictionary, JMDictEntry},
    jndict::dict::{JNDictDictionary, JNDictEntry},
    kanjidic2::dict::{KD2Dict, KD2DictDictionary},
};

pub fn parse_jmdict(path: String) -> JMDict {
    //let parser = JMDictParser::new(path, Some(String::from("en")));
    let parser = jedict_parser_configs::jmdict_parser_from_path(path, None);
    let Some(dict) = parser.parse() else {
        return JMDict::default();
    };
    JMDictDictionary::create(dict)
}
pub fn parse_jndict(path: String) -> JNDict {
    let parser = jedict_parser_configs::jndict_parser_from_path(path, None);
    let Some(dict) = parser.parse() else {
        return JNDict::default();
    };
    JNDictDictionary::create(dict)
}
pub fn parse_kanjidic2(path: String) -> KD2Dict {
    let parser = jedict_parser_configs::kanjidic2_parser_from_path(path, None);
    let Some(dict) = parser.parse() else {
        return KD2Dict::default();
    };
    KD2DictDictionary::create(dict)
}
pub fn parse_jmdict_as_json(path: String) -> String {
    parse_jmdict(path).to_json::<JMDict>()
}
pub fn parse_jndict_as_json(path: String) -> String {
    parse_jndict(path).to_json::<JNDict>()
}
pub fn parse_kanjidic2_as_json(path: String) -> String {
    parse_kanjidic2(path).to_json::<KD2Dict>()
}
pub fn jmdict_to_json(dict: JMDict) -> String {
    dict.to_json::<JMDict>()
}
pub fn jmdict_entry_to_json(entry: JMDictEntry) -> String {
    entry.to_json::<JMDictEntry>()
}
pub fn jndict_to_json(dict: JNDict) -> String {
    dict.to_json::<JNDict>()
}
pub fn jndict_entry_to_json(entry: JNDictEntry) -> String {
    entry.to_json::<JNDictEntry>()
}
pub fn kanjidic2_to_json(dict: KD2Dict) -> String {
    dict.to_json::<KD2Dict>()
}
pub fn kanjidic2_header_to_json(header: KD2DictHeader) -> String {
    header.to_json::<KD2DictHeader>()
}
pub fn kanjidic2_character_to_json(character: KD2DictCharacter) -> String {
    character.to_json::<KD2DictCharacter>()
}
pub fn parse_kanjidic2_char_from_json(json: String) -> Vec<KD2DictCharacter> {
    serde_json::from_str::<Vec<KD2DictCharacter>>(json.as_str()).unwrap_or(Vec::new())
}
pub fn parse_kanjidic2_header_from_json(json: String) -> Option<KD2DictHeader> {
    serde_json::from_str::<KD2DictHeader>(json.as_str()).ok()
}
pub fn parse_jmdict_entries_from_json(json: String) -> Vec<JMDictEntry> {
    serde_json::from_str(json.as_str()).unwrap_or(Vec::new())
}
pub fn parse_jmdict_entry_from_json(json: String) -> JMDictEntry {
    serde_json::from_str(json.as_str()).unwrap_or(JMDictEntry::default())
}
pub fn parse_jndict_entries_from_json(json: String) -> Vec<JNDictEntry> {
    serde_json::from_str(json.as_str()).unwrap_or(Vec::new())
}
pub fn parse_jndict_entry_from_json(json: String) -> JNDictEntry {
    serde_json::from_str(json.as_str()).unwrap_or(JNDictEntry::default())
}
pub fn parse_kanjidic2_entries_from_json(json: String) -> Vec<KD2DictCharacter> {
    serde_json::from_str(json.as_str()).unwrap_or(Vec::new())
}
pub fn parse_kanjidic2_entry_from_json(json: String) -> KD2DictCharacter {
    serde_json::from_str(json.as_str()).unwrap_or(KD2DictCharacter::default())
}
