use std::{path::PathBuf, collections::HashMap, sync::Arc};
use walkdir::WalkDir;
use crate::{Error, site::SiteMetadata, document::{DocumentName, Document, DocumentType}};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct File {
    pub site: Arc<SiteMetadata>,
    pub path: PathBuf,
    pub source_path: PathBuf,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FileStore {
    pub documents: HashMap<DocumentName, Document>,
    pub files: HashMap<PathBuf, File>,
}

impl FileStore {
    pub fn new(site: Arc<SiteMetadata>) -> Result<FileStore, Box<dyn std::error::Error>> {
        let mut documents = HashMap::new();
        let mut files = HashMap::new();

        let walker = WalkDir::new(&site.path).into_iter().filter_entry(|entry| {
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
                let typ = if let Some(extension) = entry.path().extension() {
                    let extension = extension.to_str().ok_or(Error::PathContainNonUnicode)?;

                    match extension {
                        "md" => Some(DocumentType::Markdown),
                        "adoc" => Some(DocumentType::AsciiDoc),
                        _ => None,
                    }
                } else {
                    None
                };

                if let Some(typ) = typ {
                    let document = Document::new(site.clone(), entry.path(), typ)?;
                    documents.insert(document.name.clone(), document);
                } else {
                    let rel_file_path = entry.path().strip_prefix(&site.path)?;

                    let file = File {
                        site: site.clone(),
                        path: rel_file_path.to_owned(),
                        source_path: entry.path().to_owned(),
                    };

                    files.insert(file.path.clone(), file);
                }
            }
        }

        Ok(FileStore { documents, files })
    }
}
