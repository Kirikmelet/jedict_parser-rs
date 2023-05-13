use crate::elements::JElementDataVariant;
use crate::elements::JElementData;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum JMElementData {
    JMDict,
    Entry,
    EntSeq,
    KEle,
    Keb,
    KeInf,
    KePri,
    REle,
    Reb,
    ReNoKanji,
    ReRestr,
    ReInf,
    RePri,
    Sense,
    Stagr,
    Stagk,
    XRef,
    Ant,
    POS,
    Field,
    Misc,
    LSource,
    Dial,
    Gloss,
    Pri,
    SInf,
    Example,
    ExSrce,
    ExText,
    ExSent,
    #[default]
    Nil,
}

impl JElementData for JMElementData {
    fn get_name(data: &JMElementData) -> String {
        match data {
            JMElementData::JMDict => "jmdict",
            JMElementData::Entry => "entry",
            JMElementData::EntSeq => "ent_seq",
            JMElementData::KEle => "k_ele",
            JMElementData::Keb => "keb",
            JMElementData::KeInf => "ke_inf",
            JMElementData::KePri => "ke_pri",
            JMElementData::REle => "r_ele",
            JMElementData::Reb => "reb",
            JMElementData::ReNoKanji => "re_nokanji",
            JMElementData::ReRestr => "re_restr",
            JMElementData::ReInf => "re_inf",
            JMElementData::RePri => "re_pri",
            JMElementData::Sense => "sense",
            JMElementData::Stagr => "stagr",
            JMElementData::Stagk => "stagk",
            JMElementData::XRef => "xref",
            JMElementData::Ant => "ant",
            JMElementData::POS => "pos",
            JMElementData::Field => "field",
            JMElementData::Misc => "misc",
            JMElementData::LSource => "lsource",
            JMElementData::Dial => "dial",
            JMElementData::Gloss => "gloss",
            JMElementData::Pri => "pri",
            JMElementData::SInf => "s_inf",
            JMElementData::Example => "example",
            JMElementData::ExSrce => "ex_srce",
            JMElementData::ExText => "ex_text",
            JMElementData::ExSent => "ex_sent",
            JMElementData::Nil => "nil",
        }
        .to_string()
    }
    fn from_name(name: String) -> Self {
        match name.to_lowercase().as_str() {
            "jmdict" => JMElementData::JMDict,
            "entry" => JMElementData::Entry,
            "ent_seq" => JMElementData::EntSeq,
            "k_ele" => JMElementData::KEle,
            "keb" => JMElementData::Keb,
            "ke_inf" => JMElementData::KeInf,
            "ke_pri" => JMElementData::KePri,
            "r_ele" => JMElementData::REle,
            "reb" => JMElementData::Reb,
            "re_nokanji" => JMElementData::ReNoKanji,
            "re_restr" => JMElementData::ReRestr,
            "re_inf" => JMElementData::ReInf,
            "re_pri" => JMElementData::RePri,
            "sense" => JMElementData::Sense,
            "stagr" => JMElementData::Stagr,
            "stagk" => JMElementData::Stagk,
            "xref" => JMElementData::XRef,
            "ant" => JMElementData::Ant,
            "pos" => JMElementData::POS,
            "field" => JMElementData::Field,
            "misc" => JMElementData::Misc,
            "lsource" => JMElementData::LSource,
            "dial" => JMElementData::Dial,
            "gloss" => JMElementData::Gloss,
            "pri" => JMElementData::Pri,
            "s_inf" => JMElementData::SInf,
            "example" => JMElementData::Example,
            "ex_srce" => JMElementData::ExSrce,
            "ex_text" => JMElementData::ExText,
            "ex_sent" => JMElementData::ExSent,
            _ => JMElementData::Nil,
        }
    }
    fn verify_type(data: &JElementDataVariant) -> Option<Self>
    where
        Self: Sized,
    {
        match data {
            JElementDataVariant::JMDict(x) => Some(x.clone()),
            _ => None,
        }
    }
}

impl ToString for JMElementData {
    fn to_string(&self) -> String {
        JMElementData::get_name(self)
    }
}
