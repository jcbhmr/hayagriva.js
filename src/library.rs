use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Library(pub(crate) hayagriva_rs::Library);

#[wasm_bindgen]
impl Library {
    pub fn new() -> Library {
        return Library {
            0: hayagriva_rs::Library::new(),
        };
    }

    pub fn push(&mut self, entry: &Entry) {
        self.0.push(&entry.0);
    }

    pub fn get(&self, key: &str) -> Option<Entry> {
        return self.0.get(key).map(|x| Entry { 0: x.clone() });
    }

    // pkg.patch
    pub fn iter(&self) -> js_sys::Iterator {
        let x = self.0.iter();
        let x = x.map(|x| Entry { 0: x.clone() });
        let x = x.collect::<Vec<Entry>>();
        let x: Array = x.into_iter().map(JsValue::from).collect();
        return x.values();
    }

    // pkg.patch
    pub fn keys(&self) -> js_sys::Iterator {
        let x = self.0.keys();
        let x = x.map(|x| x.to_string());
        let x = x.collect::<Vec<String>>();
        let x: Array = x.into_iter().map(JsValue::from).collect();
        return x.values();
    }

    pub fn remove(&mut self, key: &str) -> Option<Entry> {
        return self.0.remove(key).map(|x| Entry { 0: x.clone() });
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }

    pub fn nth(&self, n: usize) -> Option<Entry> {
        return self.0.nth(n).map(|x| Entry { 0: x.clone() });
    }
}
