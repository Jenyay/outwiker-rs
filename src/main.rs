mod ow_core;

use std::path::Path;
use std::time::Instant;

use ow_core::notetree::{Page, PageLoadingError};
use ow_core::pageengine::FilesNoteTreeEngine;

use crate::ow_core::application::Application;
use crate::ow_core::notetree::WikiDocument;
use crate::ow_core::pageengine::NoteTreeEngine;

pub fn load_note_tree(
    appliction: &mut Application,
    root_path: &str,
) -> Result<(), PageLoadingError> {
    if let Ok(document) = appliction.note_tree_engine().load_note_tree(root_path) {
        appliction.set_document(document);
        Result::Ok(())
    } else {
        Result::Err(PageLoadingError::NotFound {})
    }
}

fn print_tree(document: &WikiDocument) {
    fn print_page(page: &Page, level: usize) {
        print!("{}", " ".repeat(level * 4));

        let tags_str = page.tags().join(", ");
        println!(
            "{title} {{{page_type:?}}} [{tags}]",
            title = page.title(),
            page_type = page.page_type(),
            tags = tags_str
        );
        for rc_page in page.children() {
            print_page(&rc_page.read().unwrap(), level + 1);
        }
    }

    for rc_page in document.pages() {
        print_page(&rc_page.read().unwrap(), 0);
    }
}

fn main() {
    let wiki_path = Path::new("tests/data/samplewiki");
    //let wiki_path = Path::new("/home/jenyay/mount/zettelkasten/zettelkasten");
    let note_tree_engine: Box<dyn NoteTreeEngine> = Box::new(FilesNoteTreeEngine::new());

    let application = Application::new(note_tree_engine);

    let start = Instant::now();
    let load_result = load_note_tree(&mut application.borrow_mut(), wiki_path.to_str().unwrap());
    let duration = start.elapsed();
    println!("Время загрузки заметок: {:?}", duration);

    match load_result {
        Ok(()) => {
            print_tree(&application.borrow().document().as_ref().unwrap());
        }
        Err(err) => {
            println!("Error");
        }
    }
}
