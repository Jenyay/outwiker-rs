use std::cell::RefCell;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::{fs, io};

use crate::ow_core::notetree::{Page, PageLoadingError, WikiDocument};

pub trait PageEngine {
    fn get_context(&self, page: &Page) -> Result<String, io::Error>;
    fn load_params(&self, page: &mut Page);
    fn load_note_tree(
        &self,
        root_path: &str,
    ) -> Result<Rc<RefCell<Page>>, PageLoadingError>;
}

struct FilesPageLoader {
    context_file_name: String,
    self_weak: Weak<RefCell<Box<dyn PageEngine>>>,
}

impl FilesPageLoader {
    pub fn new() -> Rc<RefCell<Box<dyn PageEngine>>> {
        let rc_loader = Rc::new_cyclic(|weak| {
            let loader = FilesPageLoader {
                context_file_name: String::from("__page.text"),
                self_weak: weak.clone(),
            };
            let boxed: Box<dyn PageEngine> = Box::new(loader);
            RefCell::new(boxed)
        });

        rc_loader
    }

    fn _get_title(path: &String) -> String {
        let path_clear = if path.ends_with("/") {
            &path[..path.len() - 1].to_string()
        } else {
            path
        };
        match path_clear.rfind("/") {
            Some(pos) => path_clear[pos + 1..].to_string(),
            None => String::from(path_clear),
        }
    }

    fn _load_note_tree(
        &self,
        result: &mut Vec<Rc<RefCell<Page>>>,
        current_path: &str,
        root_path: &str,
        parent: Option<Weak<RefCell<Page>>>,
    ) {
        let title = Self::_get_title(&String::from(current_path));
        let page = Page::new(
            self.self_weak.clone(),
            current_path.to_string(),
            title,
            parent.clone(),
        );

        let rc_page = Rc::new(RefCell::new(page));
        if let Some(weak_parent_page) = parent {
            if let Some(parent_page) = weak_parent_page.upgrade() {
                parent_page.borrow_mut().add_child(&rc_page);
            }
        }
        result.push(rc_page);

        if let Ok(entries) = fs::read_dir(current_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && !path.to_str().unwrap().starts_with("__") {
                    let weak_rc = Rc::downgrade(result.last().unwrap());
                    self._load_note_tree(result, &path.to_str().unwrap(), root_path, Some(weak_rc));
                }
            }
        }
    }
}

impl PageEngine for FilesPageLoader {
    fn get_context(&self, page: &Page) -> Result<String, io::Error> {
        let context_file = Path::new(page.path()).join(&self.context_file_name);
        fs::read_to_string(context_file)
    }

    fn load_params(&self, page: &mut Page) {}

    fn load_note_tree(&self, root_path: &str) -> Result<Rc<RefCell<Page>>, PageLoadingError> {
        let mut result = vec![];
        self._load_note_tree(&mut result, root_path, root_path, None);
        Ok(result[0].clone())
    }
}

// PageEngineFactory
pub trait PageEngineFactory {
    fn get_page_engine(&self) -> Rc<RefCell<Box<dyn PageEngine>>>;
}

pub struct FilesPageEngineFactory {}

impl FilesPageEngineFactory {
    pub fn new() -> Self {
        FilesPageEngineFactory {}
    }
}

impl PageEngineFactory for FilesPageEngineFactory {
    fn get_page_engine(&self) -> Rc<RefCell<Box<dyn PageEngine>>> {
        FilesPageLoader::new()
    }
}
