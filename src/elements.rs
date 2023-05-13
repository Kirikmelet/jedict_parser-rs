use crate::{
    dom::DOMDocument, jmdict::elements::JMElementData, jndict::elements::JNElementData,
    kanjidic2::elements::KD2ElementData,
};
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum JElementDataVariant {
    JMDict(JMElementData),
    JNDict(JNElementData),
    KanjiDic2(KD2ElementData),
    Document(DOMDocument),
    #[default]
    Nil,
}

pub type BoxedVerifyType<T> = Box<dyn Fn(&JElementDataVariant) -> Option<T>>;
pub type BoxedFromNameType<T> = Box<dyn Fn(String) -> T>;
pub type GetNameFunc<T> = Box<dyn Fn(&T) -> String>;

impl std::fmt::Display for JElementDataVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait JElementData: Sized + ToString {
    fn get_name(data: &Self) -> String
    where
        Self: Sized;
    fn from_name(data: String) -> Self
    where
        Self: Sized;
    fn verify_type(data: &JElementDataVariant) -> Option<Self>
    where
        Self: Sized;
}

impl JElementData for JElementDataVariant {
    fn get_name(data: &Self) -> String
    where
        Self: Sized,
    {
        match data {
            Self::JMDict(..) => "meta::jmdict",
            Self::JNDict(..) => "meta::jndict",
            Self::KanjiDic2(..) => "meta::kanjidic2",
            Self::Document(..) => "document",
            Self::Nil => "nil",
        }
        .to_string()
    }

    fn from_name(data: String) -> Self
    where
        Self: Sized,
    {
        match data.to_lowercase().as_str() {
            "meta::jmdict" => Self::JMDict(JMElementData::Nil),
            "meta::kanjidic2" => Self::KanjiDic2(KD2ElementData::Nil),
            "meta::jndict" => Self::JNDict(JNElementData::Nil),
            "document" => Self::Document(DOMDocument::default()),
            _ => Self::Nil,
        }
    }

    fn verify_type(data: &JElementDataVariant) -> Option<Self>
    where
        Self: Sized,
    {
        Some(data.clone()) // No Shit sherlock!
    }
}

impl From<JMElementData> for JElementDataVariant {
    fn from(val: JMElementData) -> Self {
        JElementDataVariant::JMDict(val)
    }
}
impl From<JNElementData> for JElementDataVariant {
    fn from(value: JNElementData) -> Self {
        JElementDataVariant::JNDict(value)
    }
}
impl From<KD2ElementData> for JElementDataVariant {
    fn from(value: KD2ElementData) -> Self {
        JElementDataVariant::KanjiDic2(value)
    }
}

