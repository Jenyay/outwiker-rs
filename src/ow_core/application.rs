use crate::ow_core::{notetree::WikiDocument, pageengine::PageEngine};

pub struct Application {
    wiki_root: WikiDocument,
    page_engine: Box<dyn PageEngine>,
}
