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
}
