use std::{path::{Path, PathBuf}, collections::HashMap};
use walkdir::WalkDir;
use crate::{site::SiteName, document::{DocumentName, Document}};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FileStore {
    pub documents: HashMap<DocumentName, Document>,
    pub files: HashMap<PathBuf, ()>,
}

impl FileStore {
    pub fn new(site_name: SiteName, path: &Path) -> Result<FileStore, Box<dyn std::error::Error>> {
        let mut documents = HashMap::new();
        let mut files = HashMap::new();

        let walker = WalkDir::new(path).into_iter().filter_entry(|entry| {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name == "_posts" && entry.file_type().is_dir() {
                    return true;
                }

                if file_name.starts_with(".") || file_name.starts_with("_") {
                    return false;
                }

                return true;
            }

            return false;
        });

        for entry in walker {
            let entry = entry?;

            if entry.file_type().is_file() {
                println!("{:?}", entry.path().display());
            }
        }

        Ok(FileStore { documents, files })
    }
}
