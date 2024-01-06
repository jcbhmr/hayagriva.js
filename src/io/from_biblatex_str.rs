use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn from_biblatex_str(s: &str) -> Result<Library, Error> {
    return hayagriva_rs::io::from_biblatex_str(s).map_or_else(
        |errs| {
            Err(Error::new(
                &errs
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join("\n"),
            ))
        },
        |bib| Ok(Library { 0: bib }),
    );
}
