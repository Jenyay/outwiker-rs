use std::{fs, io};
use std::path::Path;

use crate::ow_core::notetree::{Page};

pub trait PageLoader {
    fn get_context(&self, page: &Page) -> Result<String, io::Error>;
    fn load_params(&self, page: &mut Page);
}

struct FilesPageLoader{
    context_file_name: String,
}

impl FilesPageLoader {
    pub fn new() -> Self {
        FilesPageLoader { context_file_name: String::from("__page.text") }
    }
}


impl PageLoader for FilesPageLoader {
    fn get_context(&self, page: &Page) -> Result<String, io::Error> {
        let context_file = Path::new(page.path()).join(&self.context_file_name);
        fs::read_to_string(context_file)
    }

    fn load_params(&self, page: &mut Page) {
    }
}


// PageLoaderFactory
pub trait PageLoaderFactory {
    fn get_page_loader(&self) -> Box<dyn PageLoader>;
}


pub struct FilesPageLoaderFactory{}

impl FilesPageLoaderFactory {
    pub fn new() -> Self {
        FilesPageLoaderFactory {}
    }
}

impl PageLoaderFactory for FilesPageLoaderFactory {
    fn get_page_loader(&self) -> Box<dyn PageLoader> {
        Box::new(FilesPageLoader::new())
    }    
}
