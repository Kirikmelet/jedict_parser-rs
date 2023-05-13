#![feature(lazy_cell)]

#[cfg(feature = "flutter")]
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
#[cfg(feature = "flutter")]
pub mod flutter_api;
#[cfg(feature = "flutter")]
pub mod flutter_log;
// Default
pub mod dict;
pub mod dom;
pub mod elements;
pub mod entities;
pub mod jmdict;
pub mod jndict;
pub mod kanjidic2;
pub mod parser;

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use crate::dict::DictToJson;
    use crate::flutter_api::KD2Dict;
    use crate::jmdict::dict::{JMDictDictionary, JMDict};
    
    use crate::jndict::dict::{JNDictDictionary, JNDict};
    use crate::kanjidic2::dict::KD2DictDictionary;
    use crate::parser::jedict_parser_configs;

    static INIT_LOGGER: Once = Once::new();
    static JMDICT_FILE: &str = include_str!("../tests/JMdict.xml");
    static JNDICT_FILE: &str = include_str!("../tests/JMNedict.xml");
    static KANJIDIC2_FILE: &str = include_str!("../tests/kanjidic2.xml");
    fn init_logger() {
        INIT_LOGGER.call_once(|| {
            tracing_subscriber::fmt()
                .with_writer(std::io::stdout)
                .init();
        })
    }
    #[test]
    fn ensure_jmdict_file_reads() {
        init_logger();
        let parser = jedict_parser_configs::jmdict_parser(JMDICT_FILE.to_string(), None);
        let Some(dict) = parser.parse() else {
            panic!("Empty dict!")
        };
        let entries = JMDictDictionary::create(dict);
        std::fs::write("log_jmdict.json", entries.to_json::<JMDict>());
        assert!(!entries.0.is_empty());
        //assert!(!dict.borrow().get_id().is_empty());
    }
    #[test]
    fn ensure_jndict_file_reads() {
        init_logger();
        let parser = jedict_parser_configs::jndict_parser(JNDICT_FILE.to_string(), None);
        let Some(dict) = parser.parse() else {
            panic!("Empty dict!")
        };
        let entries = JNDictDictionary::create(dict);
        assert!(!entries.0.is_empty());
        std::fs::write("log_jndict.json", entries.to_json::<JNDict>());
        //let dict_parser = JMDictDictionary::new(dict);
        //let mut entries: Vec<JMDictEntry> = Vec::new();
        //if let Ok(mut x) = dict_parser {
        //    entries = x.create()
        //}
        //assert!(!entries.is_empty());
        ////assert!(!dict.borrow().get_id().is_empty());
    }
    #[test]
    fn ensure_kanjidic2_file_reads() {
        init_logger();
        let parser = jedict_parser_configs::kanjidic2_parser(KANJIDIC2_FILE.to_string(), None);
        let Some(dict) = parser.parse() else {
            panic!("Empty dict!")
        };
        let dict_parser = KD2DictDictionary::create(dict);
        std::fs::write("log_kanjidic2.json", dict_parser.to_json::<KD2Dict>());
        assert!(!dict_parser.header.file_version.is_empty())
        //let dict_parser = JMDictDictionary::new(dict);
        //let mut entries: Vec<JMDictEntry> = Vec::new();
        //if let Ok(mut x) = dict_parser {
        //    entries = x.create()
        //}
        //assert!(!entries.is_empty());
        ////assert!(!dict.borrow().get_id().is_empty());
    }
}

