use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::ow_core::pageengine::PageEngine;

pub struct WikiDocument {
    pages: Vec<Rc<RefCell<Page>>>,
}

#[derive(Debug)]
pub struct Page {
    page_engine: Weak<Box<dyn PageEngine>>,
    path: String,
    title: String,
    page_type: Option<String>,
    uid: Option<String>,
    alias: Option<String>,
    icon: Option<String>,
    tags: Vec<String>,
    order: Option<i32>,
    creation_datetime: Option<String>,
    edit_datetime: Option<String>,
    parent: Option<Weak<RefCell<Page>>>,
    children: Vec<Rc<RefCell<Page>>>,
}

impl WikiDocument {
    pub fn new(pages: Vec<Rc<RefCell<Page>>>) -> Self {
        WikiDocument {
            pages: pages
        }
    }

    pub fn pages(&self) -> &Vec<Rc<RefCell<Page>>> {
        &self.pages
    }

    pub fn set_pages(&mut self, root: Vec<Rc<RefCell<Page>>>) {
        self.pages = root;
    }
}


impl Page {
    pub fn new(
        page_engine: Weak<Box<dyn PageEngine>>,
        path: String,
        title: String,
        parent: Option<Weak<RefCell<Page>>>,
    ) -> Self {
        let tags = vec![];
        let children = vec![];
        Page {
            page_engine,
            path: path,
            title: title,
            page_type: None,
            uid: None,
            alias: None,
            icon: None,
            tags: tags,
            order: None,
            creation_datetime: None,
            edit_datetime: None,
            parent: parent,
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

    pub fn uid(&self) -> &Option<String> {
        &self.uid
    }

    pub fn set_uid(&mut self, uid: Option<String>) {
        self.uid = uid;
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

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    pub fn order(&self) -> &Option<i32> {
        &self.order
    }

    pub fn parent(&self) -> &Option<Weak<RefCell<Page>>> {
        &self.parent
    }

    pub fn children(&self) -> &Vec<Rc<RefCell<Page>>> {
        &self.children
    }

    pub fn add_child(&mut self, rc_page: Rc<RefCell<Page>>) {
        self.children.push(rc_page);
    }

    pub fn page_type(&self) -> &Option<String> {
        &self.page_type
    }

    pub fn set_page_type(&mut self, page_type: Option<String>) {
        self.page_type = page_type;
    }
}

#[derive(Debug)]
pub enum PageLoadingError {
    NotFound,
    InvalidFormat,
}

#[cfg(test)]
mod tests {}
