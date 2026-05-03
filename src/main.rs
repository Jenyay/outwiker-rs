mod ow_core;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use ow_core::notetree::{WikiDocument, load_note_tree};
use ow_core::pageengine::{FilesPageEngineFactory, PageEngineFactory};

fn main() {
    let wiki_path = Path::new("tests/data/samplewiki");
    let page_engine_factory = FilesPageEngineFactory::new();
    let page_engine = page_engine_factory.get_page_engine();
    let document = WikiDocument::new();
    let load_result = load_note_tree(&Rc::downgrade(&document), page_engine.as_ref(), wiki_path.to_str().unwrap());
    match load_result {
        Ok(()) => {
            let document_borrowed = document.borrow();
            let roots = document_borrowed.pages();
            println!("{roots:?}");
        }
        Err(err) => {}
    }
}
