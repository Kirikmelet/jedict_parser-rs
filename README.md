# JEDICT-Parser
A JMDICT/JNDICT/KANJIDIC2 Parser in Rust

This project was created for a Flutter+Rust application.
*However, I would not use it in any professional manner*

But should work standalone. :)


```rust
use jedict_parser::dict::DictToJson;
use jedict_parser::jmdict::dict::{JMDict, JMDictDictionary};
use jedict_parser::parser::jedict_parser_configs;
static JMDICT_FILE: &str = include_str!("File path here.xml");
fn main() {
    let parser = jedict_parser_configs::jmdict_parser(JMDICT_FILE.to_string(), None);
    let Some(dict) = parser.parse() else {
        panic!("Empty dict!")
    };
    let entries = JMDictDictionary::create(dict);
    assert!(!entries.0.is_empty());
}
```
