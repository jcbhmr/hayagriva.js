use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn to_yaml_str(lib: &Library) -> Result<String, Error> {
    let res = hayagriva_rs::io::to_yaml_str(&lib.0);
    match res {
        Ok(yaml) => Ok(yaml),
        Err(err) => Err(Error::new(&format!("{}", err))),
    }
}
