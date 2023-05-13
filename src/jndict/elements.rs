use crate::elements::{JElementData, JElementDataVariant};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum JNElementData {
    JNDict,
    Entry,
    EntSeq,
    KEle,
    Keb,
    KeInf,
    KePri,
    REle,
    Reb,
    ReRestr,
    ReInf,
    RePri,
    Trans,
    NameType,
    Xref,
    TransDet,
    #[default]
    Nil,
}

impl JElementData for JNElementData {
    fn get_name(data: &Self) -> String
    where
        Self: Sized,
    {
        match data {
            Self::JNDict => "jmnedict",
            Self::Entry => "entry",
            Self::EntSeq => "ent_seq",
            Self::KEle => "k_ele",
            Self::Keb => "keb",
            Self::KeInf => "ke_inf",
            Self::KePri => "ke_pri",
            Self::REle => "r_ele",
            Self::Reb => "reb",
            Self::ReRestr => "re_restr",
            Self::ReInf => "re_inf",
            Self::RePri => "re_pri",
            Self::Trans => "trans",
            Self::NameType => "name_type",
            Self::Xref => "xref",
            Self::TransDet => "trans_det",
            Self::Nil => "nil",
        }
        .to_string()
    }

    fn from_name(data: String) -> Self
    where
        Self: Sized,
    {
        match data.to_lowercase().as_str() {
            "jmnedict" => Self::JNDict,
            "entry" => Self::Entry,
            "ent_seq" => Self::EntSeq,
            "k_ele" => Self::KEle,
            "keb" => Self::Keb,
            "ke_inf" => Self::KeInf,
            "ke_pri" => Self::KePri,
            "r_ele" => Self::REle,
            "reb" => Self::Reb,
            "re_restr" => Self::ReRestr,
            "re_inf" => Self::ReInf,
            "re_pri" => Self::RePri,
            "trans" => Self::Trans,
            "name_type" => Self::NameType,
            "xref" => Self::Xref,
            "trans_det" => Self::TransDet,
            _ => Self::Nil,
        }
    }

    fn verify_type(data: &JElementDataVariant) -> Option<Self>
    where
        Self: Sized,
    {
        match data {
            JElementDataVariant::JNDict(x) => Some(x.clone()),
            _ => None,
        }
    }
}

impl ToString for JNElementData {
    fn to_string(&self) -> String {
        JNElementData::get_name(self)
    }
}
