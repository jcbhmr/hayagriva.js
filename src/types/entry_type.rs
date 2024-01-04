use crate::*;
use hayagriva_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum EntryType {
    Article,
    Chapter,
    Entry,
    Anthos,
    Report,
    Thesis,
    Web,
    Scene,
    Artwork,
    Patent,
    Case,
    Newspaper,
    Legislation,
    Manuscript,
    Post,
    Misc,
    Performance,
    Periodical,
    Proceedings,
    Book,
    Blog,
    Reference,
    Conference,
    Anthology,
    Repository,
    Thread,
    Video,
    Audio,
    Exhibition,
    Original,
}

impl EntryType {
    pub fn from_hayagriva_rs(x: &hayagriva_rs::types::EntryType) -> Self {
        return match x {
            hayagriva_rs::types::EntryType::Article => EntryType::Article,
            hayagriva_rs::types::EntryType::Chapter => EntryType::Chapter,
            hayagriva_rs::types::EntryType::Entry => EntryType::Entry,
            hayagriva_rs::types::EntryType::Anthos => EntryType::Anthos,
            hayagriva_rs::types::EntryType::Report => EntryType::Report,
            hayagriva_rs::types::EntryType::Thesis => EntryType::Thesis,
            hayagriva_rs::types::EntryType::Web => EntryType::Web,
            hayagriva_rs::types::EntryType::Scene => EntryType::Scene,
            hayagriva_rs::types::EntryType::Artwork => EntryType::Artwork,
            hayagriva_rs::types::EntryType::Patent => EntryType::Patent,
            hayagriva_rs::types::EntryType::Case => EntryType::Case,
            hayagriva_rs::types::EntryType::Newspaper => EntryType::Newspaper,
            hayagriva_rs::types::EntryType::Legislation => EntryType::Legislation,
            hayagriva_rs::types::EntryType::Manuscript => EntryType::Manuscript,
            hayagriva_rs::types::EntryType::Post => EntryType::Post,
            hayagriva_rs::types::EntryType::Misc => EntryType::Misc,
            hayagriva_rs::types::EntryType::Performance => EntryType::Performance,
            hayagriva_rs::types::EntryType::Periodical => EntryType::Periodical,
            hayagriva_rs::types::EntryType::Proceedings => EntryType::Proceedings,
            hayagriva_rs::types::EntryType::Book => EntryType::Book,
            hayagriva_rs::types::EntryType::Blog => EntryType::Blog,
            hayagriva_rs::types::EntryType::Reference => EntryType::Reference,
            hayagriva_rs::types::EntryType::Conference => EntryType::Conference,
            hayagriva_rs::types::EntryType::Anthology => EntryType::Anthology,
            hayagriva_rs::types::EntryType::Repository => EntryType::Repository,
            hayagriva_rs::types::EntryType::Thread => EntryType::Thread,
            hayagriva_rs::types::EntryType::Video => EntryType::Video,
            hayagriva_rs::types::EntryType::Audio => EntryType::Audio,
            hayagriva_rs::types::EntryType::Exhibition => EntryType::Exhibition,
            hayagriva_rs::types::EntryType::Original => EntryType::Original,
            &_ => todo!(),
        };
    }
    pub fn to_hayagriva_rs(&self) -> hayagriva_rs::types::EntryType {
        return match self {
            EntryType::Article => hayagriva_rs::types::EntryType::Article,
            EntryType::Chapter => hayagriva_rs::types::EntryType::Chapter,
            EntryType::Entry => hayagriva_rs::types::EntryType::Entry,
            EntryType::Anthos => hayagriva_rs::types::EntryType::Anthos,
            EntryType::Report => hayagriva_rs::types::EntryType::Report,
            EntryType::Thesis => hayagriva_rs::types::EntryType::Thesis,
            EntryType::Web => hayagriva_rs::types::EntryType::Web,
            EntryType::Scene => hayagriva_rs::types::EntryType::Scene,
            EntryType::Artwork => hayagriva_rs::types::EntryType::Artwork,
            EntryType::Patent => hayagriva_rs::types::EntryType::Patent,
            EntryType::Case => hayagriva_rs::types::EntryType::Case,
            EntryType::Newspaper => hayagriva_rs::types::EntryType::Newspaper,
            EntryType::Legislation => hayagriva_rs::types::EntryType::Legislation,
            EntryType::Manuscript => hayagriva_rs::types::EntryType::Manuscript,
            EntryType::Post => hayagriva_rs::types::EntryType::Post,
            EntryType::Misc => hayagriva_rs::types::EntryType::Misc,
            EntryType::Performance => hayagriva_rs::types::EntryType::Performance,
            EntryType::Periodical => hayagriva_rs::types::EntryType::Periodical,
            EntryType::Proceedings => hayagriva_rs::types::EntryType::Proceedings,
            EntryType::Book => hayagriva_rs::types::EntryType::Book,
            EntryType::Blog => hayagriva_rs::types::EntryType::Blog,
            EntryType::Reference => hayagriva_rs::types::EntryType::Reference,
            EntryType::Conference => hayagriva_rs::types::EntryType::Conference,
            EntryType::Anthology => hayagriva_rs::types::EntryType::Anthology,
            EntryType::Repository => hayagriva_rs::types::EntryType::Repository,
            EntryType::Thread => hayagriva_rs::types::EntryType::Thread,
            EntryType::Video => hayagriva_rs::types::EntryType::Video,
            EntryType::Audio => hayagriva_rs::types::EntryType::Audio,
            EntryType::Exhibition => hayagriva_rs::types::EntryType::Exhibition,
            EntryType::Original => hayagriva_rs::types::EntryType::Original,
        };
    }
}