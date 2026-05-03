use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::ow_core::pageengine::PageEngine;

pub struct WikiDocument {
    //page_engine: Box<dyn PageEngine>,
    //self_weak: Weak<RefCell<WikiDocument>>,
    pages: Vec<Rc<RefCell<Page>>>,
}

#[derive(Debug)]
pub struct Page {
    //page_engine: Weak<RefCell<Box<dyn PageEngine>>>,
    path: String,
    title: String,
    uid: Option<String>,
    alias: Option<String>,
    icon: Option<String>,
    tags: Vec<String>,
    order: i32,
    //root: Weak<RefCell<WikiDocument>>,
    parent: Option<Weak<RefCell<Page>>>,
    children: Vec<Rc<RefCell<Page>>>,
}

impl WikiDocument {
    pub fn new() -> Rc<RefCell<Self>> {
        let rc_document = Rc::new(RefCell::new(WikiDocument {
            //page_engine: page_engine,
            //self_weak: Weak::new(),
            pages: Vec::new(),
        }));
        //let weak = Rc::downgrade(&rc_document);
        //rc_document.borrow_mut().self_weak = weak;
        rc_document
    }

    pub fn pages(&self) -> &Vec<Rc<RefCell<Page>>> {
        &self.pages
    }

    pub fn set_root(&mut self, page: Rc<RefCell<Page>>) {
        self.pages.clear();
        self.pages.push(page);
    }
}

pub fn load_note_tree(wiki_document: &Weak<RefCell<WikiDocument>>, page_engine: &dyn PageEngine, root_path: &str) -> Result<(), PageLoadingError> {
    if let Ok(page_rc) = page_engine.load_note_tree(root_path) {
        wiki_document.upgrade().unwrap().borrow_mut().set_root(page_rc);
        Result::Ok(())
    } else {
        Result::Err(PageLoadingError::NotFound {})
    }
}

impl Page {
    pub fn new(
        //root: &Weak<RefCell<WikiDocument>>,
        //page_engine: Weak<RefCell<Box<dyn PageEngine>>>,
        path: String,
        title: String,
        parent: Option<Weak<RefCell<Page>>>,
    ) -> Self {
        let tags = vec![];
        let children = vec![];
        Page {
            //page_engine,
            path: path,
            title: title,
            uid: None,
            alias: None,
            icon: None,
            tags: tags,
            order: 0,
            //root: root.clone(),
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
