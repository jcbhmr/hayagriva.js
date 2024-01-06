#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

mod entry;
mod library;

pub use entry::*;
pub use library::*;

mod citationberg;

pub mod io;
pub mod lang;
pub mod types;

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}
