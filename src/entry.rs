use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Entry(pub(crate) hayagriva_rs::Entry);

#[wasm_bindgen]
impl Entry {
    pub fn key(&self) -> String {
        return self.0.key().to_string();
    }

    #[wasm_bindgen(constructor)]
    pub fn new(key: &str, entry_type: types::EntryType) -> Self {
        return Self {
            0: hayagriva_rs::Entry::new(key, entry_type.to_hayagriva_rs()),
        };
    }

    pub fn has(&self, key: &str) -> bool {
        return self.0.has(key);
    }

    pub fn entry_type(&self) -> types::EntryType {
        return types::EntryType::from_hayagriva_rs(self.0.entry_type());
    }

    pub fn parents(&self) -> Vec<Entry> {
        let x = self.0.parents().to_vec();
        return x.into_iter().map(|x| Entry { 0: x.clone() }).collect();
    }

    pub fn title(&self) -> Option<types::FormatString> {
        return self.0.title().map(|x| types::FormatString { 0: x.clone() });
    }

    pub fn authors(&self) -> Option<Vec<types::Person>> {
        return self.0.authors().map(|x| {
            return x
                .into_iter()
                .map(|y| types::Person { 0: y.clone() })
                .collect();
        });
    }

    // pub fn
}
