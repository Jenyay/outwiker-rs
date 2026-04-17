use std::cell::RefCell;
use std::fs;
use std::path::{absolute, Path};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Page {
    path: String,
    subpath: String,
    uid: Option<String>,
    alias: Option<String>,
    icon: Option<String>,
    tags: Vec<String>,
    order: i32,
    parent: RefCell<Option<Weak<Page>>>,
    children: RefCell<Vec<Rc<Page>>>,
}

impl Page {
    pub fn new(path: String, subpath: String, parent: Option<Weak<Page>>) -> Self {
        let tags = vec![];
        let children = RefCell::new(vec![]);
        Page {
            path: path,
            subpath: subpath,
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

    pub fn parent(&self) -> &RefCell<Option<Weak<Page>>> {
        &self.parent
    }

    pub fn children(&self) -> &RefCell<Vec<Rc<Page>>> {
        &self.children
    }
}

#[derive(Debug)]
pub enum PageLoadingError {
    NotFound,
    InvalidFormat,
}

fn _get_title(subpath: &String) -> String {
    match subpath.rfind("/") {
        Some(pos) => subpath[pos + 1..].to_string(),
        None => String::from(subpath)
    }
}

fn _load_note_tree(result: &mut Vec<Page>, current_path: &Path, root_path: &Path) {
    if let Ok(entries) = fs::read_dir(current_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()  {
                let abs_root_path = absolute(root_path).unwrap();
                let abs_current_path = absolute(path.clone()).unwrap();
                let root_len = abs_root_path.to_str().unwrap().len();

                let mut subpath_str = &abs_current_path.to_str().unwrap()[root_len..];
                if subpath_str.starts_with("\\") || subpath_str.starts_with("/") {
                    subpath_str = &subpath_str[1..];
                }

                let subpath = subpath_str.replace("\\", "/");
                let title = _get_title(&subpath);

                if !title.starts_with("__") {
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

    for page in result {
        println!("{page:?}\n");
    }

    Err(PageLoadingError::NotFound)
}

#[cfg(test)]
mod tests {}
