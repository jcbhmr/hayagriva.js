use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Library(pub(crate) hayagriva_rs::Library);

#[wasm_bindgen]
impl Library {
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
