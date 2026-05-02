mod ow_core;

use std::path::Path;

use ow_core::pageengine::{FilesPageEngineFactory, PageEngineFactory};
use ow_core::notetree::WikiDocument;

fn main() {
    let wiki_path = Path::new("tests/data/samplewiki");
    let page_engine_factory = FilesPageEngineFactory::new();
    let page_engine = page_engine_factory.get_page_engine();
    let document = WikiDocument::new(page_engine);
    let laod_result = document.borrow_mut().load_note_tree(wiki_path.to_str().unwrap());
    match laod_result {
        Ok(()) => {
            let document_borrowed = document.borrow();
            let roots = document_borrowed.pages();
            println!("{roots:?}");
        },
        Err(err) => {}
    }
}
