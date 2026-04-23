mod ow_core;

use std::path::Path;

use ow_core::notetree::load_note_tree;
use ow_core::pageloader::{FilesPageLoaderFactory, PageLoaderFactory};

fn main() {
    let wiki_path = Path::new("tests/data/samplewiki");
    let page_loader_factory = FilesPageLoaderFactory::new();
    let page_loader = page_loader_factory.get_page_loader();
    let note_tree = load_note_tree(wiki_path).unwrap();

    for page in note_tree {
        println!("{page:?}\n");
    }
}
