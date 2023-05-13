use crate::elements::{JElementData, JElementDataVariant};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum KD2ElementData {
    KanjiDic2,
    Header,
    FileVersion,
    DBVersion,
    DOC,
    Character,
    Literal,
    CodePoint,
    CPValue,
    Radical,
    RadValue,
    Misc,
    Grade,
    StrokeCount,
    Variant,
    Freq,
    RadName,
    JLPT,
    DicNumber,
    DicRef,
    QueryCode,
    QCode,
    ReadingMeaning,
    RMGroup,
    Reading,
    Meaning,
    Nanori,
    #[default]
    Nil,
}

impl JElementData for KD2ElementData {
    fn get_name(data: &Self) -> String
    where
        Self: Sized,
    {
        match data {
            Self::KanjiDic2 => "kanjidic2",
            Self::Header => "header",
            Self::FileVersion => "file_version",
            Self::DBVersion => "database_version",
            Self::DOC => "date_of_creation",
            Self::Character => "character",
            Self::Literal => "literal",
            Self::CodePoint => "codepoint",
            Self::CPValue => "cp_value",
            Self::Radical => "radical",
            Self::RadValue => "rad_value",
            Self::Misc => "misc",
            Self::Grade => "grade",
            Self::StrokeCount => "stroke_count",
            Self::Variant => "variant",
            Self::Freq => "freq",
            Self::RadName => "rad_name",
            Self::JLPT => "jlpt",
            Self::DicNumber => "dic_number",
            Self::DicRef => "dic_ref",
            Self::QueryCode => "query_code",
            Self::QCode => "q_code",
            Self::ReadingMeaning => "reading_meaning",
            Self::RMGroup => "rmgroup",
            Self::Reading => "reading",
            Self::Meaning => "meaning",
            Self::Nanori => "nanori",
            Self::Nil => "nil",
        }
        .to_string()
    }

    fn from_name(data: String) -> Self
    where
        Self: Sized,
    {
        match data.to_lowercase().as_str() {
            "kanjidic2" => Self::KanjiDic2,
            "header" => Self::Header,
            "file_version" => Self::FileVersion,
            "database_version" => Self::DBVersion,
            "date_of_creation" => Self::DOC,
            "character" => Self::Character,
            "literal" => Self::Literal,
            "codepoint" => Self::CodePoint,
            "cp_value" => Self::CPValue,
            "radical" => Self::Radical,
            "rad_value" => Self::RadValue,
            "misc" => Self::Misc,
            "grade" => Self::Grade,
            "stroke_count" => Self::StrokeCount,
            "variant" => Self::Variant,
            "freq" => Self::Freq,
            "rad_name" => Self::RadName,
            "jlpt" => Self::JLPT,
            "dic_number" => Self::DicNumber,
            "dic_ref" => Self::DicRef,
            "query_code" => Self::QueryCode,
            "q_code" => Self::QCode,
            "reading_meaning" => Self::ReadingMeaning,
            "rmgroup" => Self::RMGroup,
            "reading" => Self::Reading,
            "meaning" => Self::Meaning,
            "nanori" => Self::Nanori,
            _ => Self::Nil,
        }
    }

    fn verify_type(data: &JElementDataVariant) -> Option<Self>
    where
        Self: Sized,
    {
        match data {
            JElementDataVariant::KanjiDic2(x) => Some(x.clone()),
            _ => None,
        }
    }
}

impl ToString for KD2ElementData {
    fn to_string(&self) -> String {
        Self::get_name(self)
    }
}
