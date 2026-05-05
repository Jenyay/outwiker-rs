mod ow_core;

use std::path::Path;

use ow_core::notetree::PageLoadingError;
use ow_core::pageengine::{FilesPageEngineFactory, PageEngineFactory};

use crate::ow_core::application::Application;

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
            let pages = document.as_ref().unwrap().pages();
            println!("{pages:?}");
        }
        Err(err) => {}
    }
}
