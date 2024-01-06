use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn to_yaml_str(lib: &Library) -> Result<String, Error> {
    return hayagriva_rs::io::to_yaml_str(&lib.0)
        .map_or_else(|err| Err(Error::new(&format!("{}", err))), |x| Ok(x));
}
