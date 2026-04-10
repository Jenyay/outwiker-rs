mod ow_core;

use std::path::Path;

use ow_core::notetree::load_note_tree;

fn main() {
    let wiki_path = Path::new("tests/data/samplewiki");
    let note_tree = load_note_tree(wiki_path);
}
