use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Person(pub(crate) hayagriva_rs::types::Person);
