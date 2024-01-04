use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Entry(pub(crate) hayagriva_rs::Entry);

#[wasm_bindgen]
impl Entry {
    pub fn title(&self) -> Option<types::FormatString> {
        let res = self.0.title();
        match res {
            Some(title) => Some(types::FormatString { 0: title.clone() }),
            None => None,
        }
    }

    #[wasm_bindgen]
    pub fn key(&self) -> String {
        self.0.key().into()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(key: String, entry_type: types::EntryType) -> Entry {
        Entry(hayagriva_rs::Entry::new(&key, entry_type.0))
    }

    #[wasm_bindgen]
    pub fn has(&self, key: String) -> bool {
        self.0.has(&key)
    }

    // Wrapper methods for other hayagriva_rs::Entry class methods
    #[wasm_bindgen]
    pub fn authors(&self) -> Array {
        self.0.authors().iter().map(JsValue::from).collect::<Array>()
    }

    #[wasm_bindgen]
    pub fn year(&self) -> Option<i32> {
        self.0.year()
    }

    #[wasm_bindgen]
    pub fn publication(&self) -> Option<String> {
        self.0.publication().map(|p| p.into())
    }
}
