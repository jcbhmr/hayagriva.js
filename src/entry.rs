use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Entry(pub(crate) hayagriva_rs::Entry);

#[wasm_bindgen]
impl Entry {
    #[wasm_bindgen]
    pub fn title(&self) -> JsValue {
        JsValue::from_serde(&self.0.title().as_ref().map(|t| t.clone())).unwrap_or(JsValue::UNDEFINED)
    }
}
