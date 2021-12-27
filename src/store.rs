use std::sync::Arc;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::Error;
use crate::site::{SiteName, SiteMetadata};
use crate::file::FileMetadata;
use crate::document::{RenderedDocument, DocumentName, DocumentMetadata, DocumentType};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SiteMetadataStore {
    pub sites: HashMap<SiteName, Arc<SiteMetadataStoreItem>>,
}

impl SiteMetadataStore {
    pub fn new(root_path: &Path) -> Result<SiteMetadataStore, Box<dyn std::error::Error>> {
        let mut sites = HashMap::new();

        let root_subfolders = fs::read_dir(root_path)?;

        // let tera = Arc::new(
        //     Tera::new(
        //         root_path.join("_assets/layouts/**/*.html")
        //             .to_str().ok_or(Error::PathContainNonUnicode)?
        //     )?
        // );

        // println!("{:?}", tera);

        for site_folder in root_subfolders {
            let site_folder = site_folder?;
            let site_name = site_folder
                .file_name()
                .into_string()
                .map_err(|_| Error::PathContainNonUnicode)?;

            if site_name.starts_with(".") || site_name.starts_with("_") {
                continue;
            }

            println!("[{}] Generating the data for site ...", site_name);

            let site = Arc::new(SiteMetadata::new(SiteName(site_name.clone()), &site_folder.path())?);
            let item = Arc::new(SiteMetadataStoreItem::new(site.clone())?);

            sites.insert(SiteName(site_name), item);
        }

        Ok(SiteMetadataStore { sites })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SiteMetadataStoreItem {
    pub site: Arc<SiteMetadata>,
    pub documents: HashMap<DocumentName, Arc<DocumentMetadata>>,
    pub files: HashMap<PathBuf, Arc<FileMetadata>>,
}

impl SiteMetadataStoreItem {
    pub fn new(site: Arc<SiteMetadata>) -> Result<SiteMetadataStoreItem, Box<dyn std::error::Error>> {
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
                    let document = DocumentMetadata::new(site.clone(), entry.path(), typ)?;
                    documents.insert(document.name.clone(), Arc::new(document));
                } else {
                    let rel_file_path = entry.path().strip_prefix(&site.path)?;
                    let content = fs::read(entry.path())?;

                    let file = FileMetadata {
                        site: site.clone(),
                        path: rel_file_path.to_owned(),
                        source_path: entry.path().to_owned(),
                        content,
                    };

                    files.insert(file.path.clone(), Arc::new(file));
                }
            }
        }

        Ok(SiteMetadataStoreItem { site, documents, files })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedStore {
    pub metadata: Arc<SiteMetadataStore>,
    pub documents: HashMap<SiteName, Arc<RenderedStoreItem>>,
}

impl RenderedStore {
    pub fn new(metadata: Arc<SiteMetadataStore>) -> Result<RenderedStore, Error> {
        let documents = metadata.sites.par_iter().map(|(name, site)| {
            Ok((name.clone(), Arc::new(RenderedStoreItem::new(site.clone())?)))
        }).collect::<Result<_, Error>>()?;

        Ok(RenderedStore { metadata, documents })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedStoreItem {
    pub metadata: Arc<SiteMetadataStoreItem>,
    pub documents: HashMap<DocumentName, Arc<RenderedDocument>>,
}

impl RenderedStoreItem {
    pub fn new(metadata: Arc<SiteMetadataStoreItem>) -> Result<RenderedStoreItem, Error> {
        let documents = metadata.documents.par_iter().map(|(name, document)| {
            Ok((name.clone(), Arc::new(RenderedDocument::new(document.clone())?)))
        }).collect::<Result<_, Error>>()?;

        Ok(RenderedStoreItem { metadata, documents })
    }
}
