use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::ow_core::pageengine::PageEngine;

pub struct WikiDocument {
    page_engine: Box<dyn PageEngine>,
    pages: RefCell<Vec<Rc<Page>>>,
}

#[derive(Debug)]
pub struct Page {
    path: String,
    title: String,
    uid: Option<String>,
    alias: Option<String>,
    icon: Option<String>,
    tags: Vec<String>,
    order: i32,
    parent: RefCell<Option<Weak<Page>>>,
    children: RefCell<Vec<Rc<Page>>>,
}


impl WikiDocument {
    pub fn new(page_engine: Box<dyn PageEngine>) -> Self {
        WikiDocument { page_engine: page_engine, pages: RefCell::new(Vec::new()) }
    }

    pub fn load_note_tree(&mut self, root_path: &str) -> Result<(), PageLoadingError> {
        if let Ok(page_rc) = &self.page_engine.load_note_tree(root_path) {
            self.pages.get_mut().clear();
            self.pages.get_mut().push(page_rc.clone());
            Result::Ok(())
        }
        else {
            Result::Err(PageLoadingError::NotFound{})
        }
    }

    pub fn pages(&self) -> &RefCell<Vec<Rc<Page>>> {
        &self.pages
    }
}


impl Page {
    pub fn new(path: String, title: String, parent: Option<Weak<Page>>) -> Self {
        let tags = vec![];
        let children = RefCell::new(vec![]);
        Page {
            path: path,
            title: title,
            uid: None,
            alias: None,
            icon: None,
            tags: tags,
            order: 0,
            parent: RefCell::new(parent),
            children: children,
        }
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    //pub fn subpath(&self) -> &String {
    //    &self.subpath
    //}

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn uid(&self) -> Option<&String> {
        self.uid.as_ref()
    }

    pub fn alias(&self) -> Option<&String> {
        self.alias.as_ref()
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn order(&self) -> i32 {
        self.order
    }

    pub fn parent(&self) -> &RefCell<Option<Weak<Page>>> {
        &self.parent
    }

    pub fn children(&self) -> &RefCell<Vec<Rc<Page>>> {
        &self.children
    }

    pub fn add_child(&self, page: Rc<Page>) {
        self.children.borrow_mut().push(page);
    }
}

#[derive(Debug)]
pub enum PageLoadingError {
    NotFound,
    InvalidFormat,
}


#[cfg(test)]
mod tests {}
