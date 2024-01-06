use crate::*;
use citationberg_rs;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum LongShortForm {
    Long,
    Short,
}

impl LongShortForm {
    pub fn from_citationberg_rs(x: &citationberg_rs::LongShortForm) -> Self {
        return match x {
            citationberg_rs::LongShortForm::Long => LongShortForm::Long,
            citationberg_rs::LongShortForm::Short => LongShortForm::Short,
        };
    }
    pub fn to_citationberg_rs(&self) -> citationberg_rs::LongShortForm {
        return match self {
            LongShortForm::Long => citationberg_rs::LongShortForm::Long,
            LongShortForm::Short => citationberg_rs::LongShortForm::Short,
        };
    }
}
