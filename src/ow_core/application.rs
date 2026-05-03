use crate::ow_core::{notetree::WikiDocument, pageengine::PageEngine};
use std::{cell::RefCell, rc::Rc};

pub struct Application {
    wiki_root: WikiDocument,
    page_engine: Rc<Box<dyn PageEngine>>,
}

impl Application {
    pub fn new(page_engine: Rc<Box<dyn PageEngine>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Application {
            wiki_root: WikiDocument::new(),
            page_engine,
        }))
    }

    pub fn wiki_root(&mut self) -> &mut WikiDocument {
        &mut self.wiki_root
    }

    pub fn page_engine(&self) -> &Rc<Box<dyn PageEngine>> {
        &self.page_engine
    }
}
