use serde::{Deserialize, Serialize};
use tracing::{error, warn};

use crate::{
    dict::{DictError, DictResult, DictToJson},
    dom::{DOMDocument, DictDOMItem, DictDOMRef},
    elements::JElementDataVariant,
    entities::EntityMapType,
};

use super::elements::KD2ElementData;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2Dict {
    pub header: KD2DictHeader,
    pub characters: Vec<KD2DictCharacter>,
}
impl DictToJson for KD2Dict {
}


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictHeader {
    pub file_version: String,     // File Ver. (file_version)
    pub database_version: String, // Database Version (database_version)
    pub date_of_creation: String, // Date of Creation [parse yourself!] (date_of_creation)
}
impl DictToJson for KD2DictHeader {
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictCharacter {
    pub literal: String, // Literal Chracter itself (literal)
    pub codepoint: Vec<KD2DictCharCodePointValue>, // Codepoint of character (codepoint)
    pub radical: Vec<KD2DictCharRadical>, // Radicals of character (radical)
    pub misc: KD2DictMisc, // Misc info of character (misc)
    pub dict_number: Vec<KD2DictDicNumber>, // Dictionary Info (dic_number)
    pub query_code: Vec<KD2DictQueryCode>, // Code for querying (query_code)
    pub reading_meaning: Vec<KD2DictReadingMeaning>, // Meaning of reading (reading_meaning)
}
impl DictToJson for KD2DictCharacter {
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictCharCodePointValue {
    pub data: String,    // Data proper (cp_value)
    pub cp_type: String, // Type of codepoint (jis*/ucs)
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictCharRadical {
    pub data: String,     // Data Proper (rad_value)
    pub rad_type: String, // Type of radical, (kanxgxi/nelson/etc.)
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictDicNumber {
    pub dr_type: String,     // Dictionary type (nelson/gakken/moro/etc.)
    pub m_vol: Option<i32>,  // If moro, which vol.
    pub m_page: Option<i32>, // If moro, which page
    pub data: String,        // Data proper (dic_ref)
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictQueryCode {
    pub data: String,                  // Data proper (q_code)
    pub qc_type: String,               // Type of code (skip/deroo/etc)
    pub skip_misclass: Option<String>, // if skip, which type of miss class.
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictMisc {
    pub grade: Option<i32>,
    pub stroke_count: Vec<i32>,
    pub variant: Vec<KD2DictMiscVariant>,
    pub freq: Option<i32>,
    pub radical_name: Option<String>,
    pub jlpt: Option<i32>,
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictReadingMeaning {
    pub reading_group: Vec<KD2DictRMGroup>, // Reading/Meaning Group (rmgroup)
    pub nanori: Vec<String>,                // Nanoris (nanori)
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictRMGroup {
    pub meaning: Vec<KD2DictMeaning>, // Meanings (meaning)
    pub reading: Vec<KD2DictReading>, // Readings (reading)
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictReading {
    pub data: String,     // Data proper (reading)
    pub r_type: String,   // Reading type (pinyin/korean_r/vietnam/*)
    pub on_type: String,  // If ja_on, check if jouyou or else
    pub r_status: String, // Status of kanji
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictMeaning {
    pub data: String,   // Data proper (meaning)
    pub m_lang: String, // Target language, default is (en)
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct KD2DictMiscVariant {
    pub var_type: String, // Type of variant (jis*/deroo/etc.)
    pub data: String,     // Data proper (variant)
}

#[derive(Debug, Clone)]
pub struct KD2DictDictionary;

impl KD2DictDictionary {
    #[tracing::instrument(skip(dom))]
    pub fn create(dom: DictDOMRef) -> KD2Dict {
        let dom = dom.as_ref().borrow();
        let mut characters: Vec<KD2DictCharacter> = Vec::new();
        let JElementDataVariant::Document(ref parent) = dom.object else {
            let error = DictError::throw_wrong_type(dom.object.clone(), JElementDataVariant::Document(DOMDocument::default()));
            error!({error=?error}, "oopsie");
            return KD2Dict::default();
        };
        let entities = &parent.entities;
        let mut header = KD2DictHeader::default();
        if let Some(x) = *dom.query_child(|f| f.object == KD2ElementData::Header.into()) {
            match Self::parse_header(&x, entities) {
                Ok(x) => header = *x,
                Err(x) => error!({error=?x}, "oopsie!"),
            }
        } else {
            warn!("No header?");
        };
        for item in dom.query_children(|f| f.object == KD2ElementData::Character.into()) {
            match Self::parse_character(&item, entities) {
                Ok(x) => characters.push(*x),
                Err(x) => error!({error=?x}, "oopsie!"),
            }
        }
        KD2Dict { header, characters }
    }
    pub fn parse_header(
        dom: &DictDOMItem,
        _entities: &EntityMapType,
    ) -> DictResult<Box<KD2DictHeader>> {
        let JElementDataVariant::KanjiDic2(KD2ElementData::Header) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), KD2ElementData::Header.into()))
        };
        let mut header = Box::<KD2DictHeader>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let data = item.data.clone();
            let JElementDataVariant::KanjiDic2(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), KD2ElementData::Nil.into()))
            };
            match object {
                KD2ElementData::FileVersion => {
                    header.file_version = data;
                }
                KD2ElementData::DBVersion => {
                    header.database_version = data;
                }
                KD2ElementData::DOC => header.date_of_creation = data,
                _ => {}
            }
        }
        Ok(header)
    }
    pub fn parse_character(
        dom: &DictDOMItem,
        entities: &EntityMapType,
    ) -> DictResult<Box<KD2DictCharacter>> {
        let JElementDataVariant::KanjiDic2(KD2ElementData::Character) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), KD2ElementData::Character.into()));
        };
        let mut character = Box::<KD2DictCharacter>::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let data = item.data.clone();
            let JElementDataVariant::KanjiDic2(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), KD2ElementData::Nil.into()))
            };
            match object {
                KD2ElementData::Literal => character.literal = data,
                KD2ElementData::CodePoint => match Self::parse_codepoint(&item, entities) {
                    Ok(x) => character.codepoint.extend(x),
                    Err(x) => return Err(x),
                },
                KD2ElementData::Radical => match Self::parse_radical(&item, entities) {
                    Ok(x) => character.radical.extend(x),
                    Err(x) => return Err(x),
                },
                KD2ElementData::Misc => match Self::parse_misc(&item, entities) {
                    Ok(x) => character.misc = *x,
                    Err(x) => return Err(x),
                },
                KD2ElementData::DicNumber => match Self::parse_dic_number(&item, entities) {
                    Ok(x) => character.dict_number.extend(x),
                    Err(x) => return Err(x),
                },
                KD2ElementData::QueryCode => match Self::parse_query_code(&item, entities) {
                    Ok(x) => character.query_code.extend(x),
                    Err(x) => return Err(x),
                },
                _ => {}
            }
        }
        Ok(character)
    }
    pub fn parse_codepoint(
        dom: &DictDOMItem,
        _entities: &EntityMapType,
    ) -> DictResult<Vec<KD2DictCharCodePointValue>> {
        let JElementDataVariant::KanjiDic2(KD2ElementData::CodePoint) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), KD2ElementData::CodePoint.into()));
        };
        let mut codepoint = Vec::new();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let JElementDataVariant::KanjiDic2(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), KD2ElementData::Nil.into()));
            };
            if let KD2ElementData::CPValue = object {
                let data = item.data.clone();
                let attributes = &item.attributes;
                let cp_type = &String::default();
                let cp_type = attributes
                    .get(&String::from("cp_type"))
                    .unwrap_or(cp_type)
                    .to_owned();
                codepoint.push(KD2DictCharCodePointValue { data, cp_type });
            } else {
                warn!({object=?object.clone()}, "Why is this here?");
            }
        }
        Ok(codepoint)
    }
    pub fn parse_radical(
        dom: &DictDOMItem,
        _entities: &EntityMapType,
    ) -> DictResult<Vec<KD2DictCharRadical>> {
        let JElementDataVariant::KanjiDic2(KD2ElementData::Radical) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), KD2ElementData::CodePoint.into()));
        };
        let mut radical = Vec::new();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let data = item.data.clone();
            let attributes = &item.attributes;
            let JElementDataVariant::KanjiDic2(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), KD2ElementData::Nil.into()));
            };
            if let KD2ElementData::RadValue = object {
                let rad_type = &String::default();
                let rad_type = attributes
                    .get(&String::from("rad_type"))
                    .unwrap_or(rad_type)
                    .to_owned();
                radical.push(KD2DictCharRadical { data, rad_type });
            } else {
                warn!({object=?object.clone()}, "Why is this here?");
            }
        }
        Ok(radical)
    }
    pub fn parse_misc(
        dom: &DictDOMItem,
        _entities: &EntityMapType,
    ) -> DictResult<Box<KD2DictMisc>> {
        let JElementDataVariant::KanjiDic2(KD2ElementData::Misc) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), KD2ElementData::Misc.into()));
        };
        let mut misc: Box<KD2DictMisc> = Box::default();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let data = item.data.clone();
            let JElementDataVariant::KanjiDic2(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), KD2ElementData::Nil.into()));
            };
            match object {
                KD2ElementData::Grade => misc.grade = data.parse::<i32>().ok(),
                KD2ElementData::StrokeCount => match data.parse::<i32>() {
                    Ok(x) => misc.stroke_count.push(x),
                    Err(x) => error!({error=?x}, "Wrong type, expected i32"),
                },
                KD2ElementData::Variant => {
                    let attributes = &item.attributes;
                    let var_type = match attributes.get(&String::from("var_type")) {
                        Some(x) => String::from(x),
                        None => {
                            return Err(DictError::throw_generic(String::from("Missing var_type")))
                        }
                    };
                    misc.variant.push(KD2DictMiscVariant { data, var_type })
                }
                KD2ElementData::Freq => misc.freq = data.parse::<i32>().ok(),
                KD2ElementData::RadName => {
                    misc.radical_name = if data.is_empty() { None } else { Some(data) }
                }
                KD2ElementData::JLPT => misc.jlpt = data.parse::<i32>().ok(),
                _ => {}
            }
        }
        Ok(misc)
    }
    pub fn parse_dic_number(
        dom: &DictDOMItem,
        _entities: &EntityMapType,
    ) -> DictResult<Vec<KD2DictDicNumber>> {
        let JElementDataVariant::KanjiDic2(KD2ElementData::DicNumber) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), KD2ElementData::DicNumber.into()))
        };
        let mut dic_number: Vec<KD2DictDicNumber> = Vec::new();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let JElementDataVariant::KanjiDic2(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), KD2ElementData::Nil.into()))
            };
            if let KD2ElementData::DicRef = object {
                let data = item.data.clone();
                let attributes = &item.attributes;
                //let mut dr_type: String = String::default();
                let mut m_vol: Option<i32> = None;
                let mut m_page: Option<i32> = None;
                let dr_type = match attributes.get(&String::from("dr_type")) {
                    Some(x) => x.to_string(),
                    None => return Err(DictError::throw_generic(String::from("missing dr_type!"))),
                };
                if dr_type.eq("moro") {
                    if let Some(x) = attributes.get(&String::from("m_vol")) {
                        m_vol = x.parse::<i32>().ok()
                    }
                    if let Some(x) = attributes.get(&String::from("m_page")) {
                        m_page = x.parse::<i32>().ok()
                    }
                }
                dic_number.push(KD2DictDicNumber {
                    dr_type,
                    data,
                    m_page,
                    m_vol,
                });
            } else {
                warn!({object=?object.clone()},"Why is this here?")
            }
        }
        Ok(dic_number)
    }
    pub fn parse_query_code(
        dom: &DictDOMItem,
        _entities: &EntityMapType,
    ) -> DictResult<Vec<KD2DictQueryCode>> {
        let JElementDataVariant::KanjiDic2(KD2ElementData::QueryCode) = dom.object else {
            return Err(DictError::throw_wrong_type(dom.object.clone(), KD2ElementData::QueryCode.into()))
        };
        let mut query_code: Vec<KD2DictQueryCode> = Vec::new();
        for item in &dom.children {
            let item = item.as_ref().borrow();
            let JElementDataVariant::KanjiDic2(ref object) = item.object else {
                return Err(DictError::throw_wrong_type(item.object.clone(), KD2ElementData::Nil.into()))
            };
            if let KD2ElementData::QCode = object {
                let data = item.data.clone();
                let attributes = &item.attributes;
                let mut skip_misclass: Option<String> = None;
                let qc_type = match attributes.get(&String::from("qc_type")) {
                    Some(x) => x.to_string(),
                    None => return Err(DictError::throw_generic(String::from("Missing qc_type!"))),
                };
                if qc_type.eq("skip") {
                    if let Some(x) = attributes.get(&String::from("skip_misclass")) {
                        skip_misclass = Some(String::from(x))
                    }
                }
                query_code.push(KD2DictQueryCode {
                    data,
                    qc_type,
                    skip_misclass,
                })
            } else {
                warn!({object=?object.clone()},"Why is this here?")
            }
        }
        Ok(query_code)
    }
}
