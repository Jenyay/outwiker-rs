mod ow_core;

use std::cell::RefCell;
use std::path::Path;
use std::rc::{Rc, Weak};

use ow_core::notetree::{PageLoadingError, WikiDocument};
use ow_core::pageengine::{FilesPageEngineFactory, PageEngine, PageEngineFactory};

pub fn load_note_tree(
    wiki_document: &Weak<RefCell<WikiDocument>>,
    page_engine: &dyn PageEngine,
    root_path: &str,
) -> Result<(), PageLoadingError> {
    if let Ok(page_rc) = page_engine.load_note_tree(root_path) {
        wiki_document
            .upgrade()
            .unwrap()
            .borrow_mut()
            .set_root(page_rc);
        Result::Ok(())
    } else {
        Result::Err(PageLoadingError::NotFound {})
    }
}

fn main() {
    let wiki_path = Path::new("tests/data/samplewiki");
    let page_engine_factory = FilesPageEngineFactory::new();
    let page_engine_rc = page_engine_factory.get_page_engine();
    let document = WikiDocument::new();

    let load_result = load_note_tree(
        &Rc::downgrade(&document),
        page_engine_rc.borrow().as_ref(),
        wiki_path.to_str().unwrap(),
    );
    match load_result {
        Ok(()) => {
            let document_borrowed = document.borrow();
            let roots = document_borrowed.root();
            println!("{roots:?}");
        }
        Err(err) => {}
    }
}
