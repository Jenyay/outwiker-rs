use std::cell::RefCell;
use std::fs;
use std::path::{absolute, Path};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Page {
    path: String,
    subpath: String,
    title: String,
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
        let title = Self::_get_title(&subpath);
        Page {
            path: path,
            subpath: subpath,
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

    fn _get_title(subpath: &String) -> String {
        let subpath_clear = if subpath.ends_with("/") { &subpath[..subpath.len() - 1].to_string() } else { subpath };
        match subpath_clear.rfind("/") {
            Some(pos) => subpath_clear[pos + 1..].to_string(),
            None => String::from(subpath_clear)
        }
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn subpath(&self) -> &String {
        &self.subpath
    }

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

fn _load_note_tree(result: &mut Vec<Rc<Page>>, current_path: &Path, root_path: &Path, parent: Option<Weak<Page>>) {
    if let Ok(entries) = fs::read_dir(current_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let abs_root_path = absolute(root_path).unwrap();
                let abs_current_path = absolute(path.clone()).unwrap();
                let root_len = abs_root_path.to_str().unwrap().len();

                let mut subpath_str = &abs_current_path.to_str().unwrap()[root_len..];
                if subpath_str.starts_with("\\") || subpath_str.starts_with("/") {
                    subpath_str = &subpath_str[1..];
                }

                let subpath = subpath_str.replace("\\", "/");
                let page = Page::new(path.to_str().unwrap().to_string(), subpath, parent.clone());
                if !page.title().starts_with("__") {
                    let rc_page = Rc::new(page);
                    let weak_rc = Rc::downgrade(&rc_page);
                    if let Some(ref option_parent_page) = parent {
                        if let Some(parent_page) = option_parent_page.upgrade() {
                            parent_page.add_child(Rc::clone(&rc_page));
                        }
                    }
                    result.push(rc_page);
                    _load_note_tree(result, &path, root_path, Some(weak_rc));
                }
            }
        }
    }
}

pub fn load_note_tree(root_path: &Path) -> Result<Vec<Rc<Page>>, PageLoadingError> {
    let mut result: Vec<Rc<Page>> = vec![];
    _load_note_tree(&mut result, root_path, root_path, None);
    Ok(result)
}

#[cfg(test)]
mod tests {}
