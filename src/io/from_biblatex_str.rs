use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn from_biblatex_str(s: &str) -> Result<Library, Error> {
    let res = hayagriva_rs::io::from_biblatex_str(s);
    match res {
        Ok(bib) => Ok(Library { 0: bib }),
        Err(errs) => Err(Error::new(
            &errs
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join("\n"),
        )),
    }
}
