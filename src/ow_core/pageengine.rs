use std::cell::RefCell;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use std::{fs, io};
use serde::{Deserialize, Serialize};
//use chrono::{DateTime, Local};

use crate::ow_core::notetree::{Page, PageLoadingError, WikiDocument};

pub trait PageEngine {
    fn get_context(&self, page: &Page) -> Result<String, io::Error>;
    fn load_params(&self, page: &mut Page);
    fn load_note_tree(&self, root_path: &str) -> Result<WikiDocument, PageLoadingError>;
}

struct FilesPageLoader {
    params_file_name: String,
    context_file_name: String,
    self_weak: Weak<Box<dyn PageEngine>>,
}

//#[derive(Debug, Serialize, Deserialize)]
//struct TomlPageConfig {
//    general: General,
//}

//#[derive(Debug, Serialize, Deserialize)]
//struct General {
//    #[serde(rename = "type")]
//    type_field: Option<String>,
//    datetime: Option<String>,
//    cursorposition: Option<i64>,
//    tags: Option<String>,
//}


impl FilesPageLoader {
    pub fn new() -> Rc<Box<dyn PageEngine>> {
        let rc_loader = Rc::new_cyclic(|weak| {
            let loader = FilesPageLoader {
                params_file_name: String::from("__page.opt"),
                context_file_name: String::from("__page.text"),
                self_weak: weak.clone(),
            };
            let boxed: Box<dyn PageEngine> = Box::new(loader);
            boxed
        });

        rc_loader
    }

    fn _get_title(path: &str) -> String {
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

    fn _get_child_dirs(root_path: &str) -> Vec<String> {
        let mut result = vec![];

        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                let next_page_title = Self::_get_title(path_str);

                if path.is_dir() && !next_page_title.starts_with("__") {
                    result.push(String::from_str(path_str).unwrap());
                }
            }
        }

        result
    }

    fn _load_page(
        &self,
        root_path: &str,
        parent: Option<Weak<RefCell<Page>>>,
        page_path: &str,
        all_pages: &mut Vec<Rc<RefCell<Page>>>,
    ) -> Option<Rc<RefCell<Page>>> {
        let title = Self::_get_title(page_path);
        let page = Page::new(
            self.self_weak.clone(),
            page_path.to_string(),
            title,
            parent.clone(),
        );

        let rc_page = Rc::new(RefCell::new(page));
        all_pages.push(rc_page.clone());
        for path in Self::_get_child_dirs(page_path) {
            if let Some(rc_child_page) =
                self._load_page(root_path, Some(Rc::downgrade(&rc_page)), &path, all_pages)
            {
                rc_page.borrow_mut().add_child(rc_child_page);
            }
        }

        Some(rc_page)
    }

    fn _load_pages_params(&self, all_pages: &mut Vec<Rc<RefCell<Page>>>) {
        for rc_page in all_pages {
            self.load_params(&mut rc_page.borrow_mut());
        }
    }
}

impl PageEngine for FilesPageLoader {
    fn get_context(&self, page: &Page) -> Result<String, io::Error> {
        let context_file = Path::new(page.path()).join(&self.context_file_name);
        fs::read_to_string(context_file)
    }

    fn load_params(&self, page: &mut Page) {
        let params_file_name = Path::new(page.path()).join(&self.params_file_name);

        match fs::read_to_string(params_file_name.to_str().unwrap()) {
            Result::Ok(toml_text) => {
                //let config: TomlPageConfig = toml::from_str(toml_text)?;
            },
            Result::Err(err) => {}
        }
    }

    fn load_note_tree(&self, root_path: &str) -> Result<WikiDocument, PageLoadingError> {
        let mut root_pages: Vec<Rc<RefCell<Page>>> = vec![];
        let mut all_pages: Vec<Rc<RefCell<Page>>> = vec![];

        for path in Self::_get_child_dirs(root_path) {
            if let Some(rc_page) = self._load_page(root_path, None, &path, &mut all_pages) {
                root_pages.push(rc_page.clone());
            }
        }

        self._load_pages_params(&mut all_pages);

        Ok(WikiDocument::new(root_pages))
    }
}

// PageEngineFactory
pub trait PageEngineFactory {
    fn get_page_engine(&self) -> Rc<Box<dyn PageEngine>>;
}

pub struct FilesPageEngineFactory {}

impl FilesPageEngineFactory {
    pub fn new() -> Self {
        FilesPageEngineFactory {}
    }
}

impl PageEngineFactory for FilesPageEngineFactory {
    fn get_page_engine(&self) -> Rc<Box<dyn PageEngine>> {
        FilesPageLoader::new()
    }
}
