mod ow_core;

use std::path::Path;

use ow_core::notetree::{PageLoadingError, Page};
use ow_core::pageengine::{FilesPageEngineFactory, PageEngineFactory};

use crate::ow_core::application::Application;
use crate::ow_core::notetree::WikiDocument;

pub fn load_note_tree(
    appliction: &mut Application,
    root_path: &str,
) -> Result<(), PageLoadingError> {
    if let Ok(document) = appliction.page_engine().load_note_tree(root_path) {
        appliction.set_document(document);
        Result::Ok(())
    } else {
        Result::Err(PageLoadingError::NotFound {})
    }
}

fn print_tree(document: &WikiDocument) {
    fn print_page(page: &Page, level: usize) {
        print!("{}", " ".repeat(level * 4));
        println!("{}", page.title());
        for rc_page in page.children() {
            print_page(&rc_page.borrow(), level + 1);
        }
    }

    for rc_page in document.pages() {
        print_page(&rc_page.borrow(), 0);
    }
}

fn main() {
    let wiki_path = Path::new("tests/data/samplewiki");
    let page_engine_factory = FilesPageEngineFactory::new();
    let page_engine_rc = page_engine_factory.get_page_engine();

    let application = Application::new(page_engine_rc);

    let load_result = load_note_tree(&mut application.borrow_mut(), wiki_path.to_str().unwrap());
    match load_result {
        Ok(()) => {
            let app_borrowed = application.borrow();
            let document = app_borrowed.document();
            print_tree(&application.borrow().document().as_ref().unwrap());
        }
        Err(err) => {}
    }
}
