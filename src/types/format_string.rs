use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[repr(transparent)]
pub struct FormatString(pub(crate) hayagriva_rs::types::FormatString);

#[wasm_bindgen]
impl FormatString {
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        return self.0.value.clone().to_string();
    }
}
