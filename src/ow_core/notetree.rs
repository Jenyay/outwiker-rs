use std::fs;
use std::path::Path;

pub struct Page {
    subpath: String,
    uid: Option<String>,
    alias: Option<String>,
    icon: Option<String>,
    tags: Vec<String>,
    order: i32,
    parent: Box<Page>,
    children: Vec<Page>,
}


impl Page {
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

    pub fn parent(&self) -> Option<&Page> {
        Some(self.parent.as_ref())
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

pub fn load_note_tree(path: &Path) -> Result<Vec<Page>, PageLoadingError> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    println!("{}", name.to_string_lossy());
                }
                load_note_tree(&path);
            }
        }
    }

    Err(PageLoadingError::NotFound)
}


#[cfg(test)]
mod tests {

}
