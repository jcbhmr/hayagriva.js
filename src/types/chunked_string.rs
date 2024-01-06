use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct ChunkedString(pub(crate) hayagriva_rs::types::ChunkedString);

#[wasm_bindgen]
impl ChunkedString {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        return Self {
            0: hayagriva_rs::types::ChunkedString::new(),
        };
    }

    pub fn push_str(&mut self, s: &str, kind: types::ChunkKind) {
        self.0.push_str(s, kind.to_hayagriva_rs());
    }

    pub fn push_char(&mut self, s: char, kind: types::ChunkKind) {
        self.0.push_char(s, kind.to_hayagriva_rs());
    }
}
