use crate::ow_core::{notetree::WikiDocument, pageengine::NoteTreeEngine};
use std::{cell::RefCell, rc::Rc};

pub struct Application {
    document: Option<WikiDocument>,
    note_tree_engine: Box<dyn NoteTreeEngine>,
}

impl Application {
    pub fn new(note_tree_engine: Box<dyn NoteTreeEngine>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Application {
            document: None,
            note_tree_engine,
        }))
    }

    pub fn document(&self) -> &Option<WikiDocument> {
        &self.document
    }

    pub fn set_document(&mut self, document: WikiDocument) {
        self.document = Some(document)
    }

    pub fn note_tree_engine(&self) -> &Box<dyn NoteTreeEngine> {
        &self.note_tree_engine
    }
}
