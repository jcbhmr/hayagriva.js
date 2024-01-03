use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn from_yaml_str(s: &str) -> Result<Library, Error> {
    let res = hayagriva_rs::io::from_yaml_str(s);
    match res {
        Ok(bib) => Ok(Library { 0: bib }),
        Err(err) => Err(Error::new(&format!("{}", err))),
    }
}
