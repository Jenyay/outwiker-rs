pub struct Page {
    subpath: Str,
    uid: Option<Str>,
    alias: Option<Str>,
    icon: Option<Str>,
    tags: Vec<Str>,
    order: i32,
    parent: Box<Page>,
    children: Vec<Page>,
}


impl Page {
    pub fn subpath(&self) -> &Str {
        &self.subpath
    }

    pub fn uid(&self) -> Option<&Str> {
        self.uid.as_ref()
    }

    pub fn alias(&self) -> Option<&Str> {
        self.alias.as_ref()
    }

    pub fn icon(&self) -> Option<&Str> {
        self.icon.as_ref()
    }

    pub fn tags(&self) -> &[Str] {
        &self.tags
    }

    pub fn order(&self) -> i32 {
        self.order
    }

    pub fn parent(&self) -> Option<&Page> {
        self.parent.as_deref()
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

pub fn load_note_tree(rootdir: &str) -> Result<Vec[Page], PageLoadingError> {
    Err(PageLoadingError::NotFound)
}


#[cfg(test)]
mod tests {

}
