use std::{collections::HashMap, fmt::Debug, rc::Rc};

use quick_xml::{events::Event, Reader};
use tracing::{debug, error, warn};

use crate::elements::JElementData;
use crate::{
    dom::{DOMDocument, DictDOMRef, DictDomBuilder},
    elements::JElementDataVariant,
    entities::generate_entity_map,
};

type BoxedVerifyType<T> = Box<dyn Fn(&JElementDataVariant) -> Option<T>>;
type BoxedFromNameType<T> = Box<dyn Fn(String) -> T>;

pub struct JDictParser<T>
where
    T: Debug + Default + JElementData,
{
    file_contents: String,
    language: Option<String>,
    from_name_func: BoxedFromNameType<T>,
    verify_type_func: BoxedVerifyType<T>,
}

impl<T> Debug for JDictParser<T>
where
    T: Debug + JElementData + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TestDictParser{{file_contents={:?},language={:?}}}",
            self.file_contents, self.language,
        )
    }
}

impl<T> JDictParser<T>
where
    T: JElementData + Debug + Default,
    JElementDataVariant: From<T>,
{
    #[tracing::instrument(skip(self))] // It's too long :)
    pub fn parse(&self) -> Option<DictDOMRef> {
        let mut buf: Vec<u8> = Vec::new(); // Buffer for XML data
        let mut reader = Reader::from_str(self.file_contents.as_str());
        reader.trim_text(true);
        let document = DictDomBuilder::create()
            .id(String::from("document"))
            .object(JElementDataVariant::Document(DOMDocument::default()))
            .build();
        let mut depth_refs: Vec<DictDOMRef> = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::DocType(x)) => {
                    if let JElementDataVariant::Document(ref mut doc) =
                        document.as_ref().borrow_mut().object
                    {
                        let doc_text =
                            String::from_utf8(x.to_vec()).unwrap_or_else(|_| String::default());
                        let split_file = generate_entity_map(doc_text);
                        doc.entities = if !split_file.is_empty() {
                            split_file
                        } else {
                            warn!("Entities not found?");
                            HashMap::new()
                        };
                    }
                }
                Ok(Event::Start(x)) => {
                    let parent = depth_refs.last().unwrap_or(&document);
                    let name = x.local_name().as_ref().to_vec();
                    let name = String::from_utf8(name)
                        .unwrap_or_else(|_| String::from("nil"))
                        .to_lowercase();
                    let object = (self.from_name_func)(name.clone());
                    debug!({ element = name.clone(), obj_type = ?object }, "starting element");
                    let attr: HashMap<String, String> = x
                        .attributes()
                        .filter_map(|f| f.ok())
                        .map(|f| {
                            let key = String::from_utf8(f.key.as_ref().to_vec())
                                .unwrap_or_else(|_| format!("{:?}", f));
                            let value = String::from_utf8(f.value.to_vec())
                                .unwrap_or_else(|_| String::default());
                            (key, value)
                        })
                        .collect();
                    debug!({attributes=?attr}, "Attributes of: {name}", name=name.clone());
                    let object = DictDomBuilder::create()
                        .object(object.into())
                        .parent(parent)
                        .attributes(attr)
                        .build();
                    depth_refs.push(Rc::clone(&object));
                }
                Ok(Event::End(x)) => {
                    let name = x.local_name().as_ref().to_vec();
                    let name = String::from_utf8(name)
                        .unwrap_or_else(|_| String::from("nil"))
                        .to_lowercase();
                    let last_object = Rc::clone(depth_refs.last().unwrap_or(&document));
                    let last_object_obj = &last_object.as_ref().borrow().object;
                    let Some(last_object) = (self.verify_type_func)(last_object_obj) else {
                        error!({data=?last_object}, "Wrong parser!");
                        return None
                    };
                    if last_object.to_string() == name {
                        debug!({ element = name.clone() }, "ending element");
                        depth_refs.pop();
                    } else {
                        error!(
                            "END ELEMENT DOES NOT MATCH LAST ELEMENT! {:?} : {:?}",
                            last_object_obj, name
                        );
                        return None;
                    }
                }
                Ok(Event::Text(x)) => {
                    let Some(current_item) = depth_refs.last_mut() else {
                        error!("depth_refs has nothing!");
                        return None
                    };
                    let data = x.to_vec();
                    let data = String::from_utf8(data);
                    if let Ok(data) = data {
                        debug!({ data = data.clone() }, "has data");
                        current_item.as_ref().borrow_mut().data = data;
                    }
                }
                Err(e) => error!(error = ?e),
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }
        Some(document)
    }
}

impl<T> Default for JDictParser<T>
where
    T: Default + Debug + JElementData,
{
    fn default() -> Self {
        let from_name_func: BoxedFromNameType<T> = Box::new(|_x| T::default());
        let verify_type_func: BoxedVerifyType<T> = Box::new(|_x| None);
        Self {
            file_contents: String::default(),
            language: None,
            from_name_func,
            verify_type_func,
        }
    }
}

#[derive(Debug)]
pub struct JDictParserBuilder<T: Debug + Default + JElementData>(JDictParser<T>);

impl<T> JDictParserBuilder<T>
where
    T: JElementData + Debug + Default,
{
    pub fn create() -> Self {
        Self(JDictParser::<T>::default())
    }
    pub fn language(self, language: Option<String>) -> Self {
        Self(JDictParser { language, ..self.0 })
    }
    pub fn file_contents(self, file_contents: String) -> Self {
        Self(JDictParser {
            file_contents,
            ..self.0
        })
    }
    pub fn file_path(self, path: String) -> Self {
        let file_contents = match std::fs::read_to_string(path) {
            Ok(x) => x,
            Err(_) => String::default(),
        };
        Self(JDictParser {
            file_contents,
            ..self.0
        })
    }
    pub fn from_name_func(self, from_name_func: BoxedFromNameType<T>) -> Self {
        Self(JDictParser {
            from_name_func,
            ..self.0
        })
    }
    pub fn verify_type_func(self, verify_type_func: BoxedVerifyType<T>) -> Self {
        Self(JDictParser {
            verify_type_func,
            ..self.0
        })
    }
    pub fn build(self) -> JDictParser<T> {
        self.0
    }
}

pub mod jedict_parser_configs {
    use crate::elements::JElementData;
    use crate::kanjidic2::elements::KD2ElementData;
    use crate::{jmdict::elements::JMElementData, jndict::elements::JNElementData};

    use super::{JDictParser, JDictParserBuilder};
    pub fn jmdict_parser(
        file_contents: String,
        language: Option<String>,
    ) -> JDictParser<JMElementData> {
        JDictParserBuilder::<JMElementData>::create()
            .file_contents(file_contents)
            .language(language)
            .from_name_func(Box::new(JMElementData::from_name))
            .verify_type_func(Box::new(JMElementData::verify_type))
            .build()
    }
    pub fn jmdict_parser_from_path(
        path: String,
        language: Option<String>,
    ) -> JDictParser<JMElementData> {
        JDictParserBuilder::<JMElementData>::create()
            .file_path(path)
            .language(language)
            .from_name_func(Box::new(JMElementData::from_name))
            .verify_type_func(Box::new(JMElementData::verify_type))
            .build()
    }
    pub fn jndict_parser(
        file_contents: String,
        language: Option<String>,
    ) -> JDictParser<JNElementData> {
        JDictParserBuilder::<JNElementData>::create()
            .file_contents(file_contents)
            .language(language)
            .from_name_func(Box::new(JNElementData::from_name))
            .verify_type_func(Box::new(JNElementData::verify_type))
            .build()
    }
    pub fn jndict_parser_from_path(
        path: String,
        language: Option<String>,
    ) -> JDictParser<JNElementData> {
        JDictParserBuilder::<JNElementData>::create()
            .file_path(path)
            .language(language)
            .from_name_func(Box::new(JNElementData::from_name))
            .verify_type_func(Box::new(JNElementData::verify_type))
            .build()
    }
    pub fn kanjidic2_parser(
        file_contents: String,
        language: Option<String>,
    ) -> JDictParser<KD2ElementData> {
        JDictParserBuilder::<KD2ElementData>::create()
            .file_contents(file_contents)
            .language(language)
            .from_name_func(Box::new(KD2ElementData::from_name))
            .verify_type_func(Box::new(KD2ElementData::verify_type))
            .build()
    }
    pub fn kanjidic2_parser_from_path(
        path: String,
        language: Option<String>,
    ) -> JDictParser<KD2ElementData> {
        JDictParserBuilder::<KD2ElementData>::create()
            .file_path(path)
            .language(language)
            .from_name_func(Box::new(KD2ElementData::from_name))
            .verify_type_func(Box::new(KD2ElementData::verify_type))
            .build()
    }
}
