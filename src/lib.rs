use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

mod bibliography_driver;
mod entry;
mod library;

pub use bibliography_driver::*;
pub use entry::*;
pub use library::*;

pub mod io;
pub mod types;

#[wasm_bindgen(start)]
fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
