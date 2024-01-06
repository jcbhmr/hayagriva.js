use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct SentenceCase(pub(crate) hayagriva_rs::lang::SentenceCase);

#[wasm_bindgen]
impl SentenceCase {
    #[wasm_bindgen(getter)]
    pub fn capitalize_words_with_caps_inside(&self) -> bool {
        return self.0.capitalize_words_with_caps_inside;
    }
    #[wasm_bindgen(getter)]
    pub fn do_not_format_after_dot(&self) -> bool {
        return self.0.do_not_format_after_dot;
    }
    #[wasm_bindgen(getter)]
    pub fn keep_all_uppercase_words(&self) -> bool {
        return self.0.keep_all_uppercase_words;
    }
    #[wasm_bindgen(getter)]
    pub fn use_exception_dictionary(&self) -> bool {
        return self.0.use_exception_dictionary;
    }
    #[wasm_bindgen(getter)]
    pub fn trim_start(&self) -> bool {
        return self.0.trim_start;
    }
    #[wasm_bindgen(getter)]
    pub fn trim_end(&self) -> bool {
        return self.0.trim_end;
    }
}

#[wasm_bindgen]
impl SentenceCase {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return Self {
            0: hayagriva_rs::lang::SentenceCase::new(),
        };
    }
}
