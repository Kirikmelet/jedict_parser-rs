use serde::{Serialize, Deserialize};
use tracing::{error, warn};

use crate::{
    dom::{DictDOMItem, DictDOMRef},
    entities::EntityMapType, dict::{DictError, self, DictResult, DictToJson},
};
use crate::{elements::JElementDataVariant, dom::DOMDocument};

use super::elements::JMElementData;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JMDict(pub Vec<JMDictEntry>);

impl DictToJson for JMDict {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JMDictEntry {
    pub entry_id: i32,             // Entry ID (entry)
    pub kanji: Option<JMDictKanji>,  // Japanese Text (k_ele)
    pub reading: Vec<JMDictReading>, // Reading/Kana (r_ele)
    pub sense: Vec<JMDictSense>,     // Sense (sense)
}
impl DictToJson for JMDictEntry {}

/* The sense element will record the translational equivalent
 * of the Japanese word, plus other related information. Where there
 * are several distinctly different meanings of the word, multiple
 * sense elements will be employed.
 */
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JMDictSense {
    pub restricted_kanji: Vec<String>, // (stagk)
    pub restricted_reading: Vec<String>, // (stagr)
    pub pos: Vec<String>, // Part of Speech (pos)
    pub xref: Vec<String>, // XRef (xref)
    pub antonym: Vec<String>, // Antonym (ant)
    pub field: Vec<String>, // Field (field)
    pub misc: Vec<String>, // Misc. (misc)
    pub info: Vec<String>, // Info (s_info)
    pub langsource: Vec<JMDictSenseLangSource>, // Language Source (lsource)
    pub dialect: Vec<String>, // Dialect (dial)
    pub gloss: Vec<JMDictSenseGloss>, // Gloss //gloss
    pub example: Vec<JMDictExample>, // Example
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JMDictSenseLangSource {
    pub language: String, // Language (xml:lang)
    pub langsource_type: String, // LS Type, default to 'full' (ls_type)
    pub wasei: bool, // Check if Wasei, if 'y' set true (ls_wasei)
    pub data: String // Data
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JMDictSenseGloss {
    /* Gender Irrelevent (g_gend) */
    pub language: String, // Language (xml:lang)
    pub gloss_type: String, // Gloss Type (g_type)
    pub data: String // Data
}


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JMDictKanji {
    pub phrase: String,        // Reading (keb)
    pub info: Vec<String>,     // Info (ke_inf)
    pub priority: Vec<String>, // Priority (ke_pri)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JMDictReading {
    pub phrase: String,           // Reading (reb)
    pub no_kanji: Option<String>, // No Kanji (re_nokanji)
    pub subset: Vec<String>,      // Subset (re_restr)
    pub info: Vec<String>,        // Info (re_inf)
    pub priority: Vec<String>,    // Priority (re_pri)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct JMDictExample {
    pub source: String, // (ex_srce)
    pub text: String, // (ex_text)
    pub sentences: Vec<String> // (ex_sent)
}

#[derive(Debug, Clone)]
pub struct JMDictDictionary;


impl JMDictDictionary {
#[tracing::instrument(skip(dom))]
    pub fn create(dom: DictDOMRef) -> JMDict {
        let dom = dom.as_ref().borrow();
        let JElementDataVariant::Document(ref parent_object) = &dom.object else {
            let error = 
                DictError::throw_wrong_type(
                        dom.object.clone(),
                        JElementDataVariant::Document(DOMDocument::default()));
            error!({error=?error}, "ERROR!");
            return JMDict::default()
        };
        let entities = &parent_object.entities;
        let mut entries: Vec<JMDictEntry> = Vec::new();
        for i in dom.query_children(|f| f.object == JMElementData::Entry.into()) {
            match Self::parse_entry(&i, entities) {
                Ok(x) => {
                    entries.push(*x);
                },
                Err(i) => error!({error = ?i},"{}", format!("{:?}",entries).as_str())
            }
        }
        JMDict(entries)
    }
#[tracing::instrument]
    pub fn parse_entry(dom: &DictDOMItem, entities: &EntityMapType) -> DictResult<Box<JMDictEntry>> {
        let JElementDataVariant::JMDict(ref object) = dom.object else {
            return Err(DictError::throw_wrong_type(
                dom.object.clone(),
                JMElementData::Entry.into(),
            ));
        };
        let JMElementData::Entry = object else {
            return Err(DictError::throw_wrong_type(
                    object.clone().into(),
                JMElementData::Entry.into(),
            ));
        };
        let mut entry = Box::<JMDictEntry>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let JElementDataVariant::JMDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(
                        item.object.clone(),
                    JMElementData::Entry.into(),
                )); 
            };
            match object {
                JMElementData::EntSeq => {
                    match item.data.as_str().parse::<i32>() {
                        Ok(x) => entry.entry_id = x,
                        Err(x) => {
                            return Err(DictError::throw_generic(x.to_string()))
                        }
                    }
                }
                JMElementData::KEle => {
                    match Self::parse_kanji(&item, entities) {
                        Ok(x) => entry.kanji = Some(*x),
                        Err(x) =>  error!(error = ?x)
                    }
                }
                JMElementData::REle => {
                    match Self::parse_reading(&item, entities) {
                        Ok(x) => entry.reading.push(*x),
                        Err(x) =>  error!(error = ?x)
                    }
                }
                JMElementData::Sense => {
                    match Self::parse_sense(&item, entities) {
                        Ok(x) => entry.sense.push(*x),
                        Err(x) =>  error!(error = ?x)
                    }
                }
                _ => {}
            };
        }
        Ok(entry)
    }
#[tracing::instrument]
    pub fn parse_kanji(dom: &DictDOMItem, entities: &EntityMapType) -> DictResult<Box<JMDictKanji>> {
        let JElementDataVariant::JMDict(JMElementData::KEle) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), JMElementData::KEle.into()))
        };
        let mut kanji = Box::<JMDictKanji>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let data = item.data.clone();
            let JElementDataVariant::JMDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), JMElementData::Nil.into()))
            };
            match object {
                JMElementData::Keb => kanji.phrase = data,
                JMElementData::KeInf => kanji.info.push(dict::get_entity(data, entities)),
                JMElementData::KePri => kanji.priority.push(data),
                _ => {}
            }
        }
        Ok(kanji)
    }
#[tracing::instrument]
    pub fn parse_reading(dom: &DictDOMItem, entities: &EntityMapType) -> DictResult<Box<JMDictReading>> {
        let JElementDataVariant::JMDict(JMElementData::REle) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), JMElementData::REle.into()))
        };
        let mut read = Box::<JMDictReading>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let data = item.data.clone();
            let JElementDataVariant::JMDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), JMElementData::Nil.into()))
            };
            match object {
                JMElementData::Reb => read.phrase = data,
                JMElementData::ReNoKanji => read.no_kanji = Some(data),
                JMElementData::ReRestr => read.subset.push(data),
                JMElementData::ReInf => read.info.push(dict::get_entity(data, entities)),
                JMElementData::RePri => read.priority.push(data),
                _ => {}
            }
        }
        Ok(read)
    }
#[tracing::instrument]
    pub fn parse_sense(dom: &DictDOMItem, entities: &EntityMapType) -> DictResult<Box<JMDictSense>> {
        let JElementDataVariant::JMDict(JMElementData::Sense) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), JElementDataVariant::JMDict(JMElementData::Sense))) 
        };
        let mut sense = Box::<JMDictSense>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let data = item.data.clone();
            let attributes = &item.attributes;
            let JElementDataVariant::JMDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), JMElementData::Nil.into()))
            };
            match object {
                JMElementData::Stagk => sense.restricted_kanji.push(data),
                JMElementData::Stagr => sense.restricted_reading.push(data),
                //JMElementData::POS => sense.pos.push(data),
                JMElementData::POS => sense.pos.push(dict::get_entity(data, entities)),
                JMElementData::XRef => sense.xref.push(data),
                JMElementData::Ant => sense.antonym.push(data),
                JMElementData::Field => sense.field.push(dict::get_entity(data, entities)),
                JMElementData::Misc => sense.misc.push(dict::get_entity(data, entities)),
                JMElementData::SInf => sense.info.push(data),
                JMElementData::LSource => {
                    let language = String::from("eng");
                    let langsource_type = String::from("full");
                    let language = attributes.get(&String::from("xml:lang")).unwrap_or(&language).to_owned();
                    let langsource_type = attributes.get(&String::from("ls_type")).unwrap_or(&langsource_type).to_owned();
                    let wasei = attributes.get(&String::from("ls_wasei")).is_some();
                    let langsource = JMDictSenseLangSource {
                        language,
                        langsource_type,
                        wasei,
                        data,
                    };
                    sense.langsource.push(langsource);
                },
                JMElementData::Dial => sense.dialect.push(dict::get_entity(data, entities)),
                JMElementData::Gloss|JMElementData::Pri => {
                    let language = String::from("eng");
                    let gloss_type = String::default();
                    let language = attributes.get(&String::from("xml:lang")).unwrap_or(&language).to_owned();
                    let gloss_type = attributes.get(&String::from("g_type")).unwrap_or(&gloss_type).to_owned();
                    let gloss = JMDictSenseGloss {
                        data,
                        gloss_type,
                        language,
                    };
                    sense.gloss.push(gloss)
                },
                JMElementData::Example => {
                    match Self::parse_example(&item, entities) {
                        Ok(x) => sense.example.push(*x),
                        Err(x) =>  error!(error = ?x)
                    }
                }
                _ => {}
            }
        }
        Ok(sense)
    }
#[tracing::instrument]
    pub fn parse_example(dom: &DictDOMItem, _entities: &EntityMapType) -> DictResult<Box<JMDictExample>> {
        let JElementDataVariant::JMDict(JMElementData::Example) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), JMElementData::Example.into()))
        };
        let mut example = Box::<JMDictExample>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let data = item.data.clone();
            let JElementDataVariant::JMDict(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), JMElementData::Nil.into()))
            };
            match object {
                JMElementData::ExSrce => example.source = data,
                JMElementData::ExText => example.text = data,
                JMElementData::ExSent => example.sentences.push(data),
                _ => {}
            }
        }
        Ok(example)
    }
}

impl std::fmt::Display for JMDictDictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
