use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_parse_jmdict(port_: i64, path: *mut wire_uint_8_list) {
    wire_parse_jmdict_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_parse_jndict(port_: i64, path: *mut wire_uint_8_list) {
    wire_parse_jndict_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_parse_kanjidic2(port_: i64, path: *mut wire_uint_8_list) {
    wire_parse_kanjidic2_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_parse_jmdict_as_json(port_: i64, path: *mut wire_uint_8_list) {
    wire_parse_jmdict_as_json_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_parse_jndict_as_json(port_: i64, path: *mut wire_uint_8_list) {
    wire_parse_jndict_as_json_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_parse_kanjidic2_as_json(port_: i64, path: *mut wire_uint_8_list) {
    wire_parse_kanjidic2_as_json_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_jmdict_to_json(port_: i64, dict: *mut wire_JMDict) {
    wire_jmdict_to_json_impl(port_, dict)
}

#[no_mangle]
pub extern "C" fn wire_jmdict_entry_to_json(port_: i64, entry: *mut wire_JMDictEntry) {
    wire_jmdict_entry_to_json_impl(port_, entry)
}

#[no_mangle]
pub extern "C" fn wire_jndict_to_json(port_: i64, dict: *mut wire_JNDict) {
    wire_jndict_to_json_impl(port_, dict)
}

#[no_mangle]
pub extern "C" fn wire_jndict_entry_to_json(port_: i64, entry: *mut wire_JNDictEntry) {
    wire_jndict_entry_to_json_impl(port_, entry)
}

#[no_mangle]
pub extern "C" fn wire_kanjidic2_to_json(port_: i64, dict: *mut wire_KD2Dict) {
    wire_kanjidic2_to_json_impl(port_, dict)
}

#[no_mangle]
pub extern "C" fn wire_kanjidic2_header_to_json(port_: i64, header: *mut wire_KD2DictHeader) {
    wire_kanjidic2_header_to_json_impl(port_, header)
}

#[no_mangle]
pub extern "C" fn wire_kanjidic2_character_to_json(
    port_: i64,
    character: *mut wire_KD2DictCharacter,
) {
    wire_kanjidic2_character_to_json_impl(port_, character)
}

#[no_mangle]
pub extern "C" fn wire_parse_kanjidic2_char_from_json(port_: i64, json: *mut wire_uint_8_list) {
    wire_parse_kanjidic2_char_from_json_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_parse_kanjidic2_header_from_json(port_: i64, json: *mut wire_uint_8_list) {
    wire_parse_kanjidic2_header_from_json_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_parse_jmdict_entries_from_json(port_: i64, json: *mut wire_uint_8_list) {
    wire_parse_jmdict_entries_from_json_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_parse_jmdict_entry_from_json(port_: i64, json: *mut wire_uint_8_list) {
    wire_parse_jmdict_entry_from_json_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_parse_jndict_entries_from_json(port_: i64, json: *mut wire_uint_8_list) {
    wire_parse_jndict_entries_from_json_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_parse_jndict_entry_from_json(port_: i64, json: *mut wire_uint_8_list) {
    wire_parse_jndict_entry_from_json_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_parse_kanjidic2_entries_from_json(port_: i64, json: *mut wire_uint_8_list) {
    wire_parse_kanjidic2_entries_from_json_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_parse_kanjidic2_entry_from_json(port_: i64, json: *mut wire_uint_8_list) {
    wire_parse_kanjidic2_entry_from_json_impl(port_, json)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_jm_dict_0() -> *mut wire_JMDict {
    support::new_leak_box_ptr(wire_JMDict::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_jm_dict_entry_0() -> *mut wire_JMDictEntry {
    support::new_leak_box_ptr(wire_JMDictEntry::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_jm_dict_kanji_0() -> *mut wire_JMDictKanji {
    support::new_leak_box_ptr(wire_JMDictKanji::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_jn_dict_0() -> *mut wire_JNDict {
    support::new_leak_box_ptr(wire_JNDict::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_jn_dict_entry_0() -> *mut wire_JNDictEntry {
    support::new_leak_box_ptr(wire_JNDictEntry::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_jn_dict_kanji_0() -> *mut wire_JNDictKanji {
    support::new_leak_box_ptr(wire_JNDictKanji::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_kd_2_dict_0() -> *mut wire_KD2Dict {
    support::new_leak_box_ptr(wire_KD2Dict::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_kd_2_dict_character_0() -> *mut wire_KD2DictCharacter {
    support::new_leak_box_ptr(wire_KD2DictCharacter::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_kd_2_dict_header_0() -> *mut wire_KD2DictHeader {
    support::new_leak_box_ptr(wire_KD2DictHeader::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_int_32_list_0(len: i32) -> *mut wire_int_32_list {
    let ans = wire_int_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_jm_dict_entry_0(len: i32) -> *mut wire_list_jm_dict_entry {
    let wrap = wire_list_jm_dict_entry {
        ptr: support::new_leak_vec_ptr(<wire_JMDictEntry>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jm_dict_example_0(len: i32) -> *mut wire_list_jm_dict_example {
    let wrap = wire_list_jm_dict_example {
        ptr: support::new_leak_vec_ptr(<wire_JMDictExample>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jm_dict_reading_0(len: i32) -> *mut wire_list_jm_dict_reading {
    let wrap = wire_list_jm_dict_reading {
        ptr: support::new_leak_vec_ptr(<wire_JMDictReading>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jm_dict_sense_0(len: i32) -> *mut wire_list_jm_dict_sense {
    let wrap = wire_list_jm_dict_sense {
        ptr: support::new_leak_vec_ptr(<wire_JMDictSense>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jm_dict_sense_gloss_0(len: i32) -> *mut wire_list_jm_dict_sense_gloss {
    let wrap = wire_list_jm_dict_sense_gloss {
        ptr: support::new_leak_vec_ptr(<wire_JMDictSenseGloss>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jm_dict_sense_lang_source_0(
    len: i32,
) -> *mut wire_list_jm_dict_sense_lang_source {
    let wrap = wire_list_jm_dict_sense_lang_source {
        ptr: support::new_leak_vec_ptr(<wire_JMDictSenseLangSource>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jn_dict_entry_0(len: i32) -> *mut wire_list_jn_dict_entry {
    let wrap = wire_list_jn_dict_entry {
        ptr: support::new_leak_vec_ptr(<wire_JNDictEntry>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jn_dict_reading_0(len: i32) -> *mut wire_list_jn_dict_reading {
    let wrap = wire_list_jn_dict_reading {
        ptr: support::new_leak_vec_ptr(<wire_JNDictReading>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jn_dict_trans_det_0(len: i32) -> *mut wire_list_jn_dict_trans_det {
    let wrap = wire_list_jn_dict_trans_det {
        ptr: support::new_leak_vec_ptr(<wire_JNDictTransDet>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_jn_dict_translations_0(len: i32) -> *mut wire_list_jn_dict_translations {
    let wrap = wire_list_jn_dict_translations {
        ptr: support::new_leak_vec_ptr(<wire_JNDictTranslations>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_char_code_point_value_0(
    len: i32,
) -> *mut wire_list_kd_2_dict_char_code_point_value {
    let wrap = wire_list_kd_2_dict_char_code_point_value {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictCharCodePointValue>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_char_radical_0(
    len: i32,
) -> *mut wire_list_kd_2_dict_char_radical {
    let wrap = wire_list_kd_2_dict_char_radical {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictCharRadical>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_character_0(len: i32) -> *mut wire_list_kd_2_dict_character {
    let wrap = wire_list_kd_2_dict_character {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictCharacter>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_dic_number_0(len: i32) -> *mut wire_list_kd_2_dict_dic_number {
    let wrap = wire_list_kd_2_dict_dic_number {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictDicNumber>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_meaning_0(len: i32) -> *mut wire_list_kd_2_dict_meaning {
    let wrap = wire_list_kd_2_dict_meaning {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictMeaning>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_misc_variant_0(
    len: i32,
) -> *mut wire_list_kd_2_dict_misc_variant {
    let wrap = wire_list_kd_2_dict_misc_variant {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictMiscVariant>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_query_code_0(len: i32) -> *mut wire_list_kd_2_dict_query_code {
    let wrap = wire_list_kd_2_dict_query_code {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictQueryCode>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_reading_0(len: i32) -> *mut wire_list_kd_2_dict_reading {
    let wrap = wire_list_kd_2_dict_reading {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictReading>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_reading_meaning_0(
    len: i32,
) -> *mut wire_list_kd_2_dict_reading_meaning {
    let wrap = wire_list_kd_2_dict_reading_meaning {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictReadingMeaning>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_kd_2_dict_rm_group_0(len: i32) -> *mut wire_list_kd_2_dict_rm_group {
    let wrap = wire_list_kd_2_dict_rm_group {
        ptr: support::new_leak_vec_ptr(<wire_KD2DictRMGroup>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<JMDict> for *mut wire_JMDict {
    fn wire2api(self) -> JMDict {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<JMDict>::wire2api(*wrap).into()
    }
}
impl Wire2Api<JMDictEntry> for *mut wire_JMDictEntry {
    fn wire2api(self) -> JMDictEntry {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<JMDictEntry>::wire2api(*wrap).into()
    }
}
impl Wire2Api<JMDictKanji> for *mut wire_JMDictKanji {
    fn wire2api(self) -> JMDictKanji {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<JMDictKanji>::wire2api(*wrap).into()
    }
}
impl Wire2Api<JNDict> for *mut wire_JNDict {
    fn wire2api(self) -> JNDict {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<JNDict>::wire2api(*wrap).into()
    }
}
impl Wire2Api<JNDictEntry> for *mut wire_JNDictEntry {
    fn wire2api(self) -> JNDictEntry {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<JNDictEntry>::wire2api(*wrap).into()
    }
}
impl Wire2Api<JNDictKanji> for *mut wire_JNDictKanji {
    fn wire2api(self) -> JNDictKanji {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<JNDictKanji>::wire2api(*wrap).into()
    }
}
impl Wire2Api<KD2Dict> for *mut wire_KD2Dict {
    fn wire2api(self) -> KD2Dict {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KD2Dict>::wire2api(*wrap).into()
    }
}
impl Wire2Api<KD2DictCharacter> for *mut wire_KD2DictCharacter {
    fn wire2api(self) -> KD2DictCharacter {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KD2DictCharacter>::wire2api(*wrap).into()
    }
}
impl Wire2Api<KD2DictHeader> for *mut wire_KD2DictHeader {
    fn wire2api(self) -> KD2DictHeader {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KD2DictHeader>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Vec<i32>> for *mut wire_int_32_list {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<JMDict> for wire_JMDict {
    fn wire2api(self) -> JMDict {
        JMDict(self.field0.wire2api())
    }
}
impl Wire2Api<JMDictEntry> for wire_JMDictEntry {
    fn wire2api(self) -> JMDictEntry {
        JMDictEntry {
            entry_id: self.entry_id.wire2api(),
            kanji: self.kanji.wire2api(),
            reading: self.reading.wire2api(),
            sense: self.sense.wire2api(),
        }
    }
}
impl Wire2Api<JMDictExample> for wire_JMDictExample {
    fn wire2api(self) -> JMDictExample {
        JMDictExample {
            source: self.source.wire2api(),
            text: self.text.wire2api(),
            sentences: self.sentences.wire2api(),
        }
    }
}
impl Wire2Api<JMDictKanji> for wire_JMDictKanji {
    fn wire2api(self) -> JMDictKanji {
        JMDictKanji {
            phrase: self.phrase.wire2api(),
            info: self.info.wire2api(),
            priority: self.priority.wire2api(),
        }
    }
}
impl Wire2Api<JMDictReading> for wire_JMDictReading {
    fn wire2api(self) -> JMDictReading {
        JMDictReading {
            phrase: self.phrase.wire2api(),
            no_kanji: self.no_kanji.wire2api(),
            subset: self.subset.wire2api(),
            info: self.info.wire2api(),
            priority: self.priority.wire2api(),
        }
    }
}
impl Wire2Api<JMDictSense> for wire_JMDictSense {
    fn wire2api(self) -> JMDictSense {
        JMDictSense {
            restricted_kanji: self.restricted_kanji.wire2api(),
            restricted_reading: self.restricted_reading.wire2api(),
            pos: self.pos.wire2api(),
            xref: self.xref.wire2api(),
            antonym: self.antonym.wire2api(),
            field: self.field.wire2api(),
            misc: self.misc.wire2api(),
            info: self.info.wire2api(),
            langsource: self.langsource.wire2api(),
            dialect: self.dialect.wire2api(),
            gloss: self.gloss.wire2api(),
            example: self.example.wire2api(),
        }
    }
}
impl Wire2Api<JMDictSenseGloss> for wire_JMDictSenseGloss {
    fn wire2api(self) -> JMDictSenseGloss {
        JMDictSenseGloss {
            language: self.language.wire2api(),
            gloss_type: self.gloss_type.wire2api(),
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<JMDictSenseLangSource> for wire_JMDictSenseLangSource {
    fn wire2api(self) -> JMDictSenseLangSource {
        JMDictSenseLangSource {
            language: self.language.wire2api(),
            langsource_type: self.langsource_type.wire2api(),
            wasei: self.wasei.wire2api(),
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<JNDict> for wire_JNDict {
    fn wire2api(self) -> JNDict {
        JNDict(self.field0.wire2api())
    }
}
impl Wire2Api<JNDictEntry> for wire_JNDictEntry {
    fn wire2api(self) -> JNDictEntry {
        JNDictEntry {
            entry_id: self.entry_id.wire2api(),
            kanji: self.kanji.wire2api(),
            reading: self.reading.wire2api(),
            translation: self.translation.wire2api(),
        }
    }
}
impl Wire2Api<JNDictKanji> for wire_JNDictKanji {
    fn wire2api(self) -> JNDictKanji {
        JNDictKanji {
            phrase: self.phrase.wire2api(),
            info: self.info.wire2api(),
            priority: self.priority.wire2api(),
        }
    }
}
impl Wire2Api<JNDictReading> for wire_JNDictReading {
    fn wire2api(self) -> JNDictReading {
        JNDictReading {
            phrase: self.phrase.wire2api(),
            subset: self.subset.wire2api(),
            info: self.info.wire2api(),
            priority: self.priority.wire2api(),
        }
    }
}
impl Wire2Api<JNDictTransDet> for wire_JNDictTransDet {
    fn wire2api(self) -> JNDictTransDet {
        JNDictTransDet {
            language: self.language.wire2api(),
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<JNDictTranslations> for wire_JNDictTranslations {
    fn wire2api(self) -> JNDictTranslations {
        JNDictTranslations {
            name_type: self.name_type.wire2api(),
            xref: self.xref.wire2api(),
            transcription: self.transcription.wire2api(),
        }
    }
}
impl Wire2Api<KD2Dict> for wire_KD2Dict {
    fn wire2api(self) -> KD2Dict {
        KD2Dict {
            header: self.header.wire2api(),
            characters: self.characters.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictCharCodePointValue> for wire_KD2DictCharCodePointValue {
    fn wire2api(self) -> KD2DictCharCodePointValue {
        KD2DictCharCodePointValue {
            data: self.data.wire2api(),
            cp_type: self.cp_type.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictCharRadical> for wire_KD2DictCharRadical {
    fn wire2api(self) -> KD2DictCharRadical {
        KD2DictCharRadical {
            data: self.data.wire2api(),
            rad_type: self.rad_type.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictCharacter> for wire_KD2DictCharacter {
    fn wire2api(self) -> KD2DictCharacter {
        KD2DictCharacter {
            literal: self.literal.wire2api(),
            codepoint: self.codepoint.wire2api(),
            radical: self.radical.wire2api(),
            misc: self.misc.wire2api(),
            dict_number: self.dict_number.wire2api(),
            query_code: self.query_code.wire2api(),
            reading_meaning: self.reading_meaning.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictDicNumber> for wire_KD2DictDicNumber {
    fn wire2api(self) -> KD2DictDicNumber {
        KD2DictDicNumber {
            dr_type: self.dr_type.wire2api(),
            m_vol: self.m_vol.wire2api(),
            m_page: self.m_page.wire2api(),
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictHeader> for wire_KD2DictHeader {
    fn wire2api(self) -> KD2DictHeader {
        KD2DictHeader {
            file_version: self.file_version.wire2api(),
            database_version: self.database_version.wire2api(),
            date_of_creation: self.date_of_creation.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictMeaning> for wire_KD2DictMeaning {
    fn wire2api(self) -> KD2DictMeaning {
        KD2DictMeaning {
            data: self.data.wire2api(),
            m_lang: self.m_lang.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictMisc> for wire_KD2DictMisc {
    fn wire2api(self) -> KD2DictMisc {
        KD2DictMisc {
            grade: self.grade.wire2api(),
            stroke_count: self.stroke_count.wire2api(),
            variant: self.variant.wire2api(),
            freq: self.freq.wire2api(),
            radical_name: self.radical_name.wire2api(),
            jlpt: self.jlpt.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictMiscVariant> for wire_KD2DictMiscVariant {
    fn wire2api(self) -> KD2DictMiscVariant {
        KD2DictMiscVariant {
            var_type: self.var_type.wire2api(),
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictQueryCode> for wire_KD2DictQueryCode {
    fn wire2api(self) -> KD2DictQueryCode {
        KD2DictQueryCode {
            data: self.data.wire2api(),
            qc_type: self.qc_type.wire2api(),
            skip_misclass: self.skip_misclass.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictReading> for wire_KD2DictReading {
    fn wire2api(self) -> KD2DictReading {
        KD2DictReading {
            data: self.data.wire2api(),
            r_type: self.r_type.wire2api(),
            on_type: self.on_type.wire2api(),
            r_status: self.r_status.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictReadingMeaning> for wire_KD2DictReadingMeaning {
    fn wire2api(self) -> KD2DictReadingMeaning {
        KD2DictReadingMeaning {
            reading_group: self.reading_group.wire2api(),
            nanori: self.nanori.wire2api(),
        }
    }
}
impl Wire2Api<KD2DictRMGroup> for wire_KD2DictRMGroup {
    fn wire2api(self) -> KD2DictRMGroup {
        KD2DictRMGroup {
            meaning: self.meaning.wire2api(),
            reading: self.reading.wire2api(),
        }
    }
}
impl Wire2Api<Vec<JMDictEntry>> for *mut wire_list_jm_dict_entry {
    fn wire2api(self) -> Vec<JMDictEntry> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JMDictExample>> for *mut wire_list_jm_dict_example {
    fn wire2api(self) -> Vec<JMDictExample> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JMDictReading>> for *mut wire_list_jm_dict_reading {
    fn wire2api(self) -> Vec<JMDictReading> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JMDictSense>> for *mut wire_list_jm_dict_sense {
    fn wire2api(self) -> Vec<JMDictSense> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JMDictSenseGloss>> for *mut wire_list_jm_dict_sense_gloss {
    fn wire2api(self) -> Vec<JMDictSenseGloss> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JMDictSenseLangSource>> for *mut wire_list_jm_dict_sense_lang_source {
    fn wire2api(self) -> Vec<JMDictSenseLangSource> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JNDictEntry>> for *mut wire_list_jn_dict_entry {
    fn wire2api(self) -> Vec<JNDictEntry> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JNDictReading>> for *mut wire_list_jn_dict_reading {
    fn wire2api(self) -> Vec<JNDictReading> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JNDictTransDet>> for *mut wire_list_jn_dict_trans_det {
    fn wire2api(self) -> Vec<JNDictTransDet> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<JNDictTranslations>> for *mut wire_list_jn_dict_translations {
    fn wire2api(self) -> Vec<JNDictTranslations> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictCharCodePointValue>> for *mut wire_list_kd_2_dict_char_code_point_value {
    fn wire2api(self) -> Vec<KD2DictCharCodePointValue> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictCharRadical>> for *mut wire_list_kd_2_dict_char_radical {
    fn wire2api(self) -> Vec<KD2DictCharRadical> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictCharacter>> for *mut wire_list_kd_2_dict_character {
    fn wire2api(self) -> Vec<KD2DictCharacter> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictDicNumber>> for *mut wire_list_kd_2_dict_dic_number {
    fn wire2api(self) -> Vec<KD2DictDicNumber> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictMeaning>> for *mut wire_list_kd_2_dict_meaning {
    fn wire2api(self) -> Vec<KD2DictMeaning> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictMiscVariant>> for *mut wire_list_kd_2_dict_misc_variant {
    fn wire2api(self) -> Vec<KD2DictMiscVariant> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictQueryCode>> for *mut wire_list_kd_2_dict_query_code {
    fn wire2api(self) -> Vec<KD2DictQueryCode> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictReading>> for *mut wire_list_kd_2_dict_reading {
    fn wire2api(self) -> Vec<KD2DictReading> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictReadingMeaning>> for *mut wire_list_kd_2_dict_reading_meaning {
    fn wire2api(self) -> Vec<KD2DictReadingMeaning> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<KD2DictRMGroup>> for *mut wire_list_kd_2_dict_rm_group {
    fn wire2api(self) -> Vec<KD2DictRMGroup> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_32_list {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JMDict {
    field0: *mut wire_list_jm_dict_entry,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JMDictEntry {
    entry_id: i32,
    kanji: *mut wire_JMDictKanji,
    reading: *mut wire_list_jm_dict_reading,
    sense: *mut wire_list_jm_dict_sense,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JMDictExample {
    source: *mut wire_uint_8_list,
    text: *mut wire_uint_8_list,
    sentences: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JMDictKanji {
    phrase: *mut wire_uint_8_list,
    info: *mut wire_StringList,
    priority: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JMDictReading {
    phrase: *mut wire_uint_8_list,
    no_kanji: *mut wire_uint_8_list,
    subset: *mut wire_StringList,
    info: *mut wire_StringList,
    priority: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JMDictSense {
    restricted_kanji: *mut wire_StringList,
    restricted_reading: *mut wire_StringList,
    pos: *mut wire_StringList,
    xref: *mut wire_StringList,
    antonym: *mut wire_StringList,
    field: *mut wire_StringList,
    misc: *mut wire_StringList,
    info: *mut wire_StringList,
    langsource: *mut wire_list_jm_dict_sense_lang_source,
    dialect: *mut wire_StringList,
    gloss: *mut wire_list_jm_dict_sense_gloss,
    example: *mut wire_list_jm_dict_example,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JMDictSenseGloss {
    language: *mut wire_uint_8_list,
    gloss_type: *mut wire_uint_8_list,
    data: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JMDictSenseLangSource {
    language: *mut wire_uint_8_list,
    langsource_type: *mut wire_uint_8_list,
    wasei: bool,
    data: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JNDict {
    field0: *mut wire_list_jn_dict_entry,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JNDictEntry {
    entry_id: i32,
    kanji: *mut wire_JNDictKanji,
    reading: *mut wire_list_jn_dict_reading,
    translation: *mut wire_list_jn_dict_translations,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JNDictKanji {
    phrase: *mut wire_uint_8_list,
    info: *mut wire_StringList,
    priority: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JNDictReading {
    phrase: *mut wire_uint_8_list,
    subset: *mut wire_StringList,
    info: *mut wire_StringList,
    priority: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JNDictTransDet {
    language: *mut wire_uint_8_list,
    data: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_JNDictTranslations {
    name_type: *mut wire_StringList,
    xref: *mut wire_StringList,
    transcription: *mut wire_list_jn_dict_trans_det,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2Dict {
    header: wire_KD2DictHeader,
    characters: *mut wire_list_kd_2_dict_character,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictCharCodePointValue {
    data: *mut wire_uint_8_list,
    cp_type: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictCharRadical {
    data: *mut wire_uint_8_list,
    rad_type: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictCharacter {
    literal: *mut wire_uint_8_list,
    codepoint: *mut wire_list_kd_2_dict_char_code_point_value,
    radical: *mut wire_list_kd_2_dict_char_radical,
    misc: wire_KD2DictMisc,
    dict_number: *mut wire_list_kd_2_dict_dic_number,
    query_code: *mut wire_list_kd_2_dict_query_code,
    reading_meaning: *mut wire_list_kd_2_dict_reading_meaning,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictDicNumber {
    dr_type: *mut wire_uint_8_list,
    m_vol: *mut i32,
    m_page: *mut i32,
    data: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictHeader {
    file_version: *mut wire_uint_8_list,
    database_version: *mut wire_uint_8_list,
    date_of_creation: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictMeaning {
    data: *mut wire_uint_8_list,
    m_lang: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictMisc {
    grade: *mut i32,
    stroke_count: *mut wire_int_32_list,
    variant: *mut wire_list_kd_2_dict_misc_variant,
    freq: *mut i32,
    radical_name: *mut wire_uint_8_list,
    jlpt: *mut i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictMiscVariant {
    var_type: *mut wire_uint_8_list,
    data: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictQueryCode {
    data: *mut wire_uint_8_list,
    qc_type: *mut wire_uint_8_list,
    skip_misclass: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictReading {
    data: *mut wire_uint_8_list,
    r_type: *mut wire_uint_8_list,
    on_type: *mut wire_uint_8_list,
    r_status: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictReadingMeaning {
    reading_group: *mut wire_list_kd_2_dict_rm_group,
    nanori: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KD2DictRMGroup {
    meaning: *mut wire_list_kd_2_dict_meaning,
    reading: *mut wire_list_kd_2_dict_reading,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jm_dict_entry {
    ptr: *mut wire_JMDictEntry,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jm_dict_example {
    ptr: *mut wire_JMDictExample,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jm_dict_reading {
    ptr: *mut wire_JMDictReading,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jm_dict_sense {
    ptr: *mut wire_JMDictSense,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jm_dict_sense_gloss {
    ptr: *mut wire_JMDictSenseGloss,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jm_dict_sense_lang_source {
    ptr: *mut wire_JMDictSenseLangSource,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jn_dict_entry {
    ptr: *mut wire_JNDictEntry,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jn_dict_reading {
    ptr: *mut wire_JNDictReading,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jn_dict_trans_det {
    ptr: *mut wire_JNDictTransDet,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_jn_dict_translations {
    ptr: *mut wire_JNDictTranslations,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_char_code_point_value {
    ptr: *mut wire_KD2DictCharCodePointValue,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_char_radical {
    ptr: *mut wire_KD2DictCharRadical,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_character {
    ptr: *mut wire_KD2DictCharacter,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_dic_number {
    ptr: *mut wire_KD2DictDicNumber,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_meaning {
    ptr: *mut wire_KD2DictMeaning,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_misc_variant {
    ptr: *mut wire_KD2DictMiscVariant,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_query_code {
    ptr: *mut wire_KD2DictQueryCode,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_reading {
    ptr: *mut wire_KD2DictReading,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_reading_meaning {
    ptr: *mut wire_KD2DictReadingMeaning,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_kd_2_dict_rm_group {
    ptr: *mut wire_KD2DictRMGroup,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_JMDict {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JMDict {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JMDictEntry {
    fn new_with_null_ptr() -> Self {
        Self {
            entry_id: Default::default(),
            kanji: core::ptr::null_mut(),
            reading: core::ptr::null_mut(),
            sense: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JMDictEntry {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JMDictExample {
    fn new_with_null_ptr() -> Self {
        Self {
            source: core::ptr::null_mut(),
            text: core::ptr::null_mut(),
            sentences: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JMDictExample {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JMDictKanji {
    fn new_with_null_ptr() -> Self {
        Self {
            phrase: core::ptr::null_mut(),
            info: core::ptr::null_mut(),
            priority: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JMDictKanji {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JMDictReading {
    fn new_with_null_ptr() -> Self {
        Self {
            phrase: core::ptr::null_mut(),
            no_kanji: core::ptr::null_mut(),
            subset: core::ptr::null_mut(),
            info: core::ptr::null_mut(),
            priority: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JMDictReading {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JMDictSense {
    fn new_with_null_ptr() -> Self {
        Self {
            restricted_kanji: core::ptr::null_mut(),
            restricted_reading: core::ptr::null_mut(),
            pos: core::ptr::null_mut(),
            xref: core::ptr::null_mut(),
            antonym: core::ptr::null_mut(),
            field: core::ptr::null_mut(),
            misc: core::ptr::null_mut(),
            info: core::ptr::null_mut(),
            langsource: core::ptr::null_mut(),
            dialect: core::ptr::null_mut(),
            gloss: core::ptr::null_mut(),
            example: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JMDictSense {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JMDictSenseGloss {
    fn new_with_null_ptr() -> Self {
        Self {
            language: core::ptr::null_mut(),
            gloss_type: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JMDictSenseGloss {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JMDictSenseLangSource {
    fn new_with_null_ptr() -> Self {
        Self {
            language: core::ptr::null_mut(),
            langsource_type: core::ptr::null_mut(),
            wasei: Default::default(),
            data: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JMDictSenseLangSource {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JNDict {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JNDict {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JNDictEntry {
    fn new_with_null_ptr() -> Self {
        Self {
            entry_id: Default::default(),
            kanji: core::ptr::null_mut(),
            reading: core::ptr::null_mut(),
            translation: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JNDictEntry {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JNDictKanji {
    fn new_with_null_ptr() -> Self {
        Self {
            phrase: core::ptr::null_mut(),
            info: core::ptr::null_mut(),
            priority: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JNDictKanji {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JNDictReading {
    fn new_with_null_ptr() -> Self {
        Self {
            phrase: core::ptr::null_mut(),
            subset: core::ptr::null_mut(),
            info: core::ptr::null_mut(),
            priority: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JNDictReading {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JNDictTransDet {
    fn new_with_null_ptr() -> Self {
        Self {
            language: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JNDictTransDet {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_JNDictTranslations {
    fn new_with_null_ptr() -> Self {
        Self {
            name_type: core::ptr::null_mut(),
            xref: core::ptr::null_mut(),
            transcription: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_JNDictTranslations {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2Dict {
    fn new_with_null_ptr() -> Self {
        Self {
            header: Default::default(),
            characters: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2Dict {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictCharCodePointValue {
    fn new_with_null_ptr() -> Self {
        Self {
            data: core::ptr::null_mut(),
            cp_type: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictCharCodePointValue {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictCharRadical {
    fn new_with_null_ptr() -> Self {
        Self {
            data: core::ptr::null_mut(),
            rad_type: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictCharRadical {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictCharacter {
    fn new_with_null_ptr() -> Self {
        Self {
            literal: core::ptr::null_mut(),
            codepoint: core::ptr::null_mut(),
            radical: core::ptr::null_mut(),
            misc: Default::default(),
            dict_number: core::ptr::null_mut(),
            query_code: core::ptr::null_mut(),
            reading_meaning: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictCharacter {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictDicNumber {
    fn new_with_null_ptr() -> Self {
        Self {
            dr_type: core::ptr::null_mut(),
            m_vol: core::ptr::null_mut(),
            m_page: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictDicNumber {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictHeader {
    fn new_with_null_ptr() -> Self {
        Self {
            file_version: core::ptr::null_mut(),
            database_version: core::ptr::null_mut(),
            date_of_creation: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictHeader {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictMeaning {
    fn new_with_null_ptr() -> Self {
        Self {
            data: core::ptr::null_mut(),
            m_lang: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictMeaning {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictMisc {
    fn new_with_null_ptr() -> Self {
        Self {
            grade: core::ptr::null_mut(),
            stroke_count: core::ptr::null_mut(),
            variant: core::ptr::null_mut(),
            freq: core::ptr::null_mut(),
            radical_name: core::ptr::null_mut(),
            jlpt: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictMisc {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictMiscVariant {
    fn new_with_null_ptr() -> Self {
        Self {
            var_type: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictMiscVariant {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictQueryCode {
    fn new_with_null_ptr() -> Self {
        Self {
            data: core::ptr::null_mut(),
            qc_type: core::ptr::null_mut(),
            skip_misclass: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictQueryCode {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictReading {
    fn new_with_null_ptr() -> Self {
        Self {
            data: core::ptr::null_mut(),
            r_type: core::ptr::null_mut(),
            on_type: core::ptr::null_mut(),
            r_status: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictReading {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictReadingMeaning {
    fn new_with_null_ptr() -> Self {
        Self {
            reading_group: core::ptr::null_mut(),
            nanori: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictReadingMeaning {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_KD2DictRMGroup {
    fn new_with_null_ptr() -> Self {
        Self {
            meaning: core::ptr::null_mut(),
            reading: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_KD2DictRMGroup {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
