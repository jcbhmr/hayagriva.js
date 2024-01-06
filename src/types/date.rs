use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Date(pub(crate) hayagriva_rs::types::Date);

#[wasm_bindgen]
impl Date {
    #[wasm_bindgen(getter)]
    pub fn year(&self) -> i32 {
        return self.0.year;
    }
    #[wasm_bindgen(getter)]
    pub fn month(&self) -> Option<u8> {
        return self.0.month;
    }
    #[wasm_bindgen(getter)]
    pub fn day(&self) -> Option<u8> {
        return self.0.day;
    }
    #[wasm_bindgen(getter)]
    pub fn approximate(&self) -> bool {
        return self.0.approximate;
    }
}

#[wasm_bindgen]
impl Date {
    pub fn from_year(year: i32) -> Self {
        return Self {
            0: hayagriva_rs::types::Date::from_year(year),
        };
    }

    pub fn display_year(&self) -> String {
        return self.0.display_year();
    }

    pub fn display_year_opt(
        &self,
        secular: bool,
        periods: bool,
        designate_positive: bool,
        ad_prefix: bool,
    ) -> String {
        return self
            .0
            .display_year_opt(secular, periods, designate_positive, ad_prefix);
    }
}
