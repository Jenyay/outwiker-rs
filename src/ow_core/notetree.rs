use std::fs;
use std::path::{Path, absolute};

pub struct Page {
    path: String,
    subpath: String,
    uid: Option<String>,
    alias: Option<String>,
    icon: Option<String>,
    tags: Vec<String>,
    order: i32,
    parent: Option<Box<Page>>,
    children: Vec<Page>,
}

impl Page {
    pub fn new(path: String, subpath: String, parent: Option<Box<Page>>) -> Self {
        let tags = vec![];
        let children = vec![];
        Page {
            path: path,
            subpath: subpath,
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

    pub fn subpath(&self) -> &String {
        &self.subpath
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

    pub fn parent(&self) -> &Option<Box<Page>> {
        &self.parent
    }

    pub fn children(&self) -> &[Page] {
        &self.children
    }
}

#[derive(Debug)]
pub enum PageLoadingError {
    NotFound,
    InvalidFormat,
}

fn _load_note_tree(result: &mut Vec<Page>, current_path: &Path, root_path: &Path) {
    if let Ok(entries) = fs::read_dir(current_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    let abs_current_path = absolute(path.clone()).unwrap();
                    let abs_root_path = absolute(root_path).unwrap();
                    let root_len = abs_root_path.to_str().unwrap().len();

                    let subpath = abs_current_path.to_str().unwrap()[root_len..].to_string();
                    let page = Page::new(path.to_str().unwrap().to_string(), subpath, None);
                    result.push(page);
                }
                _load_note_tree(result, &path, root_path);
            }
        }
    }
}

pub fn load_note_tree(root_path: &Path) -> Result<Vec<String>, PageLoadingError> {
    let mut result: Vec<Page> = vec![];
    _load_note_tree(&mut result, root_path, root_path);

    //if let Ok(entries) = fs::read_dir(root_path) {
    //    for entry in entries.flatten() {
    //        let path = entry.path();
    //        if path.is_dir() {
    //            if let Some(name) = path.file_name() {
    //                println!("{}", name.to_string_lossy());
    //            }
    //            load_note_tree(&path);
    //        }
    //    }
    //}

    Err(PageLoadingError::NotFound)
}

#[cfg(test)]
mod tests {}
