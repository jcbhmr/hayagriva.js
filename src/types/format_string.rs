use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen(getter_with_clone)]
pub struct FormatString(pub(crate) hayagriva_rs::types::FormatString);

#[wasm_bindgen]
impl FormatString {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return FormatString {
            0: hayagriva_rs::types::FormatString::new(),
        };
    }

    pub fn with_value(value: String) -> Self {
        return FormatString {
            0: hayagriva_rs::types::FormatString::with_value(value),
        };
    }

    pub fn with_short(value: String, short: String) -> Self {
        return FormatString {
            0: hayagriva_rs::types::FormatString::with_short(value, short),
        };
    }

    // pub fn fmt_short(&self) {
    //     todo!();
    // }

    pub fn format_title_case(&self, props: lang::TitleCase) -> String {
        todo!();
    }

    pub fn format_sentence_case(&self, props: lang::SentenceCase) -> String {
        todo!();
    }

    pub fn select(&self, form: citationberg::LongShortForm) -> types::ChunkedString {
        todo!();
    }
}
