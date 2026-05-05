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
    uid: Option<String>,
    alias: Option<String>,
    icon: Option<String>,
    tags: Vec<String>,
    order: i32,
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
            uid: None,
            alias: None,
            icon: None,
            tags: tags,
            order: 0,
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

    pub fn parent(&self) -> &Option<Weak<RefCell<Page>>> {
        &self.parent
    }

    pub fn children(&self) -> &Vec<Rc<RefCell<Page>>> {
        &self.children
    }

    pub fn add_child(&mut self, page: &Rc<RefCell<Page>>) {
        self.children.push(page.clone());
    }
}

#[derive(Debug)]
pub enum PageLoadingError {
    NotFound,
    InvalidFormat,
}

#[cfg(test)]
mod tests {}
