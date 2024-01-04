use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct SentenceCase(pub(crate) hayagriva_rs::lang::SentenceCase);
