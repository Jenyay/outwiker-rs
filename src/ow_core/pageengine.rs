extern crate ini;

use std::cell::RefCell;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use std::{fs, io};

use ini::{Ini, Properties};

use crate::ow_core::notetree::{Page, PageLoadingError, WikiDocument};

pub trait PageEngine {
    fn get_title(&self, path: &str) -> String;
    fn get_context(&self, page: &Page) -> Result<String, io::Error>;
    fn load_params(&self, page: &mut Page);
}

pub trait NoteTreeEngine {
    fn load_note_tree(&self, root_path: &str) -> Result<WikiDocument, PageLoadingError>;
    fn page_engine(&self) -> Weak<dyn PageEngine>;
}

struct FilesPageEngine {}

pub struct FilesNoteTreeEngine {
    page_engine: Rc<dyn PageEngine>,
}

impl FilesPageEngine {
    const PARAMS_FILE_NAME: &str = "__page.opt";
    const CONTEXT_FILE_NAME: &str = "__page.text";
    const PARAMS_SECTION_GENERAL: &str = "General";
    const PARAM_TYPE: &str = "type";
    const PARAM_TAGS: &str = "tags";
    const PARAM_UID: &str = "uid";
    const PARAM_ICON: &str = "icon";
    const PARAM_ORDER: &str = "order";
    const PARAM_CREATION_DATETIME: &str = "creationdatetime";
    const PARAM_EDIT_DATETIME: &str = "datetime";

    fn _parse_tags_string(tags_str: Option<&str>) -> Option<Vec<String>> {
        Some(
            tags_str?
                .split(",")
                .map(|item| String::from_str(&item.trim().to_lowercase()).unwrap())
                .collect(),
        )
    }

    fn _get_param_str(section: &Properties, param_name: &str) -> Option<String> {
        Some(String::from_str(section.get(param_name)?.trim()).unwrap())
    }
}

impl PageEngine for FilesPageEngine {
    fn get_context(&self, page: &Page) -> Result<String, io::Error> {
        let context_file = Path::new(page.path()).join(Self::CONTEXT_FILE_NAME);
        fs::read_to_string(context_file)
    }

    fn get_title(&self, path: &str) -> String {
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

    fn load_params(&self, page: &mut Page) {
        let params_file_name = Path::new(page.path()).join(Self::PARAMS_FILE_NAME);

        match fs::read_to_string(params_file_name.to_str().unwrap()) {
            Result::Ok(ini_text) => {
                let config = Ini::load_from_str(&ini_text).unwrap();
                let general_section: &Properties =
                    config.section(Some(Self::PARAMS_SECTION_GENERAL)).unwrap();
                page.set_page_type(Self::_get_param_str(general_section, Self::PARAM_TYPE));
                if let Some(tags) = Self::_parse_tags_string(general_section.get(Self::PARAM_TAGS))
                {
                    page.set_tags(tags);
                }
                page.set_uid(Self::_get_param_str(general_section, Self::PARAM_UID));
            }
            Result::Err(err) => {}
        }
    }
}

impl FilesNoteTreeEngine {
    pub fn new() -> FilesNoteTreeEngine {
        let page_engine_rc: Rc<dyn PageEngine> = Rc::new(FilesPageEngine {});
        FilesNoteTreeEngine {
            page_engine: page_engine_rc,
        }
    }

    fn _load_pages_params(&self, all_pages: &mut Vec<Rc<RefCell<Page>>>) {
        for rc_page in all_pages {
            self.page_engine.load_params(&mut rc_page.borrow_mut());
        }
    }

    fn _get_child_dirs(&self, root_path: &str) -> Vec<String> {
        let mut result = vec![];

        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                let next_page_title = self.page_engine.get_title(path_str);

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
        let title = self.page_engine.get_title(page_path);
        let page = Page::new(
            self.page_engine(),
            page_path.to_string(),
            title,
            parent.clone(),
        );

        let rc_page = Rc::new(RefCell::new(page));
        all_pages.push(rc_page.clone());
        for path in self._get_child_dirs(page_path) {
            if let Some(rc_child_page) =
                self._load_page(root_path, Some(Rc::downgrade(&rc_page)), &path, all_pages)
            {
                rc_page.borrow_mut().add_child(rc_child_page);
            }
        }

        Some(rc_page)
    }
}

impl NoteTreeEngine for FilesNoteTreeEngine {
    fn load_note_tree(&self, root_path: &str) -> Result<WikiDocument, PageLoadingError> {
        let mut root_pages: Vec<Rc<RefCell<Page>>> = vec![];
        let mut all_pages: Vec<Rc<RefCell<Page>>> = vec![];

        for path in self._get_child_dirs(root_path) {
            if let Some(rc_page) = self._load_page(root_path, None, &path, &mut all_pages) {
                root_pages.push(rc_page.clone());
            }
        }

        self._load_pages_params(&mut all_pages);

        Ok(WikiDocument::new(root_pages))
    }

    fn page_engine(&self) -> Weak<dyn PageEngine> {
        Rc::downgrade(&self.page_engine)
    }
}
