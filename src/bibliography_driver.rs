use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

// #[wasm_bindgen]
// #[repr(transparent)]
// pub struct BibliographyDriver<T>(pub(crate) hayagriva_rs::BibliographyDriver<'a, T>);

// #[wasm_bindgen]
// impl BibliographyDriver {
//     #[wasm_bindgen(constructor)]
//     pub fn new() {
//         return Self {
//             0: hayagriva_rs::BibliographyDriver::new(),
//         };
//     }

//     // pub fn citation(&mut self, req: CitationRequest) {
//     //     self.0.citation(req.0);
//     // }

//     // pub fn finish(self, request: BibliographyRequest) -> Rendered {
//     //     return self.0.finish(request.0);
//     // }
// }
