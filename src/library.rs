use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Library(pub(crate) hayagriva_rs::Library);

#[wasm_bindgen]
impl Library {
    #[wasm_bindgen]
    pub fn nth(&self, n: usize) -> Option<Entry> {
        self.0.nth(n).map(|entry| Entry { 0: entry.clone() })
    }
    #[wasm_bindgen]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[wasm_bindgen]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[wasm_bindgen]
    pub fn remove(&mut self, key: &str) -> Option<Entry> {
        self.0.remove(key).map(|entry| Entry { 0: entry })
    }
    #[wasm_bindgen]
    pub fn keys(&self) -> js_sys::Iterator {
        self.0.keys().map(JsValue::from).collect::<Array>().values()
    }
    #[wasm_bindgen]
    #[wasm_bindgen]
    pub fn iter(&self) -> js_sys::Iterator {
        self.0.iter().map(|entry| JsValue::into_serde(&Entry { 0: entry.clone() }).unwrap()).collect::<Array>().values()
    }
    #[wasm_bindgen]
    pub fn new() -> Library {
        Library(hayagriva_rs::Library::new())
    }
    pub fn push(&mut self, entry: &Entry) {
        self.0.push(&entry.0);
    }

    pub fn get(&self, key: &str) -> Option<Entry> {
        let res = self.0.get(key);
        match res {
            Some(entry) => Some(Entry { 0: entry.clone() }),
            None => None,
        }
    }
}
