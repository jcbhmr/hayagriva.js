use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum ChunkKind {
    Normal,
    Verbatim,
    Math,
}

impl ChunkKind {
    pub fn from_hayagriva_rs(x: &hayagriva_rs::types::ChunkKind) -> Self {
        return match x {
            hayagriva_rs::types::ChunkKind::Normal => ChunkKind::Normal,
            hayagriva_rs::types::ChunkKind::Verbatim => ChunkKind::Verbatim,
            hayagriva_rs::types::ChunkKind::Math => ChunkKind::Math,
        };
    }
    pub fn to_hayagriva_rs(&self) -> hayagriva_rs::types::ChunkKind {
        return match self {
            ChunkKind::Normal => hayagriva_rs::types::ChunkKind::Normal,
            ChunkKind::Verbatim => hayagriva_rs::types::ChunkKind::Verbatim,
            ChunkKind::Math => hayagriva_rs::types::ChunkKind::Math,
        };
    }
}
