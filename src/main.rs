mod ow_core;

use std::path::Path;

use ow_core::pageengine::{FilesPageEngineFactory, PageEngineFactory};
use ow_core::notetree::WikiDocument;

fn main() {
    let wiki_path = Path::new("tests/data/samplewiki");
    let page_engine_factory = FilesPageEngineFactory::new();
    let page_engine = page_engine_factory.get_page_engine();
    let mut document = WikiDocument::new(page_engine);
    match document.load_note_tree(wiki_path.to_str().unwrap()) {
        Ok(()) => {
            let roots = document.pages();
            println!("{roots:?}");
        },
        Err(err) => {}
    }
}
