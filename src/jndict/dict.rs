use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    dict::{self, DictError, DictResult, DictToJson},
    dom::{DOMDocument, DictDOMItem, DictDOMRef},
    elements::JElementDataVariant,
    entities::EntityMapType,
};

use super::elements::JNElementData;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JNDict(pub Vec<JNDictEntry>);

impl DictToJson for JNDict {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JNDictEntry {
    pub entry_id: i32,                        // Entity Sequence (ent_seq)
    pub kanji: Option<JNDictKanji>,           // Associated Kanji (k_ele)
    pub reading: Vec<JNDictReading>,          // Associated Reading (r_ele)
    pub translation: Vec<JNDictTranslations>, // Translations (trans)
}

impl DictToJson for JNDictEntry {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JNDictKanji {
    pub phrase: String,        // Phrase (keb)
    pub info: Vec<String>,     // Info (ke_inf)
    pub priority: Vec<String>, // Priority (ke_pri)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JNDictReading {
    pub phrase: String,        // Phrease (reb)
    pub subset: Vec<String>,   // Subset (re_restr)
    pub info: Vec<String>,     // Info (re_inf)
    pub priority: Vec<String>, // Priority (re_pri)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JNDictTranslations {
    pub name_type: Vec<String>,             // Name Type (name_type)
    pub xref: Vec<String>,                  // Xref (xref)
    pub transcription: Vec<JNDictTransDet>, // Translation Destination (trans_det)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JNDictTransDet {
    pub language: String,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct JNDictDictionary;

impl JNDictDictionary {
#[tracing::instrument(skip(dom))]
    pub fn create(dom: DictDOMRef) -> JNDict {
        let dom = dom.as_ref().borrow();
        let mut dict: Vec<JNDictEntry> = Vec::new();
        let JElementDataVariant::Document(ref parent_object) = &dom.object else {
            let error = DictError::throw_wrong_type(dom.object.clone(), JElementDataVariant::Document(DOMDocument::default()));
            error!({error=?error}, "Error!");
            return JNDict::default();
        };
        let entities = &parent_object.entities;
        for i in &dom.query_children(|f| f.object == JNElementData::Entry.into()) {
            match Self::parse_entry(i, entities) {
                Ok(x) => dict.push(*x),
                Err(x) => error!({error=?x}, "Oopsie"),
            }
        }
        JNDict(dict)
    }
    pub fn parse_entry(
        dom: &DictDOMItem,
        entities: &EntityMapType,
    ) -> DictResult<Box<JNDictEntry>> {
        let JElementDataVariant::JNDict(JNElementData::Entry) = dom.object else {
          return Err(DictError::throw_wrong_type(dom.object.clone(), JNElementData::Nil.into()));
        };
        let mut entry = Box::<JNDictEntry>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let JElementDataVariant::JNDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), JElementDataVariant::Nil))
            };
            match object {
                JNElementData::EntSeq => match item.data.as_str().parse::<i32>() {
                    Ok(x) => entry.entry_id = x,
                    Err(x) => return Err(DictError::throw_generic(x.to_string())),
                },
                JNElementData::KEle => match Self::parse_kanji(&item, entities) {
                    Ok(x) => entry.kanji = Some(*x),
                    Err(x) => error!({error=?x}, "Oopsie"),
                },
                JNElementData::REle => match Self::parse_reading(&item, entities) {
                    Ok(x) => entry.reading.push(*x),
                    Err(x) => error!({error=?x}, "Oopsie"),
                },
                JNElementData::Trans => match Self::parse_translation(&item, entities) {
                    Ok(x) => entry.translation.push(*x),
                    Err(x) => error!({error=?x}, "Oopsie"),
                },
                _ => {}
            }
        }
        Ok(entry)
    }
    pub fn parse_kanji(
        dom: &DictDOMItem,
        _entities: &EntityMapType,
    ) -> DictResult<Box<JNDictKanji>> {
        let JElementDataVariant::JNDict(JNElementData::KEle) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), JNElementData::REle.into()));
        };
        let mut kanji = Box::<JNDictKanji>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let JElementDataVariant::JNDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), JNElementData::Nil.into()))
            };
            let data = item.data.clone();
            match object {
                JNElementData::Keb => {
                    kanji.phrase = data;
                }
                JNElementData::KeInf => kanji.info.push(data),
                JNElementData::KePri => kanji.priority.push(data),
                _ => {}
            }
        }
        Ok(kanji)
    }
    pub fn parse_reading(
        dom: &DictDOMItem,
        _entities: &EntityMapType,
    ) -> DictResult<Box<JNDictReading>> {
        let JElementDataVariant::JNDict(JNElementData::REle) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), JNElementData::REle.into()));
        };
        let mut read = Box::<JNDictReading>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let JElementDataVariant::JNDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), JNElementData::Nil.into()))
            };
            let data = item.data.clone();
            match object {
                JNElementData::Reb => {
                    read.phrase = data;
                }
                JNElementData::ReInf => read.info.push(data),
                JNElementData::RePri => read.priority.push(data),
                JNElementData::ReRestr => read.subset.push(data),
                _ => {}
            }
        }
        Ok(read)
    }
    pub fn parse_translation(
        dom: &DictDOMItem,
        entities: &EntityMapType,
    ) -> DictResult<Box<JNDictTranslations>> {
        let JElementDataVariant::JNDict(JNElementData::Trans) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), JNElementData::Trans.into()));
        };
        let mut trans = Box::<JNDictTranslations>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let JElementDataVariant::JNDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), JNElementData::Nil.into()))
            };
            let data = item.data.clone();
            match object {
                JNElementData::NameType => trans.xref.push(dict::get_entity(data, entities)),
                JNElementData::Xref => trans.xref.push(data),
                JNElementData::TransDet => {
                    let language = &String::from("eng");
                    let language = item
                        .attributes
                        .get(&String::from("xml:lang"))
                        .unwrap_or(language)
                        .to_owned();
                    let dest = JNDictTransDet { language, data };
                    trans.transcription.push(dest);
                }
                _ => {}
            }
        }
        Ok(trans)
    }
}
