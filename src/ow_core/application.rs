use crate::ow_core::{notetree::WikiDocument, pageengine::PageEngine};
//use std::{cell::RefCell, rc::Rc};
use std::sync::Arc;
use std::cell::RefCell;

pub struct Application {
    document: Option<WikiDocument>,
    page_engine: Arc<Box<dyn PageEngine>>,
}

impl Application {
    pub fn new(page_engine: Arc<Box<dyn PageEngine>>) -> Arc<RefCell<Self>> {
        Arc::new(RefCell::new(Application {
            document: None,
            page_engine,
        }))
    }

    pub fn document(&self) -> &Option<WikiDocument> {
        &self.document
    }

    pub fn set_document(&mut self, document: WikiDocument) {
        self.document = Some(document)
    }

    pub fn page_engine(&self) -> &Arc<Box<dyn PageEngine>> {
        &self.page_engine
    }
}
