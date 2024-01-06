use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct TitleCase(pub(crate) hayagriva_rs::lang::TitleCase);

#[wasm_bindgen]
impl TitleCase {
    #[wasm_bindgen(getter)]
    pub fn always_capitalize_after_punctuation(&self) -> bool {
        return self.0.always_capitalize_after_punctuation;
    }
    #[wasm_bindgen(getter)]
    pub fn always_capitalize_last_word(&self) -> bool {
        return self.0.always_capitalize_last_word;
    }
    #[wasm_bindgen(getter)]
    pub fn always_capitalize_min_len(&self) -> Option<usize> {
        return self.0.always_capitalize_min_len;
    }
    #[wasm_bindgen(getter)]
    pub fn hyphen_word_separator(&self) -> bool {
        return self.0.hyphen_word_separator;
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
impl TitleCase {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return Self {
            0: hayagriva_rs::lang::TitleCase::new(),
        };
    }
}
