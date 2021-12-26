use std::path::{Path, PathBuf, Component};
use crate::{Error, site::SiteName};

#[derive(Hash, Eq, Clone, PartialEq, Debug)]
pub struct DocumentName {
    pub labels: Vec<String>,
    pub post: Option<DocumentPostLabel>,
}

#[derive(Hash, Eq, Clone, PartialEq, Debug)]
pub struct DocumentPostLabel {
    pub date: String,
    pub label: String,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub enum DocumentType {
    AsciiDoc,
    Markdown,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Document {
    pub site: SiteName,
    pub name: DocumentName,
    pub typ: DocumentType,
    pub source_path: PathBuf,
}

impl Document {
    pub fn new(
        site: SiteName,
        site_path: &Path,
        file_path: &Path,
        typ: DocumentType,
    ) -> Result<Document, Box<dyn std::error::Error>> {
        let rel_file_path = file_path.strip_prefix(site_path)?;

        let mut labels = Vec::new();
        let mut is_post = false;
        let mut components = rel_file_path.components().peekable();

        while let Some(component) = components.next() {
            if let Component::Normal(component_name) = component {
                let component_name = component_name.to_str().ok_or(Error::PathContainNonUnicode)?;

                if component_name == "_posts" {
                    is_post = true;
                    break
                }

                if components.peek().is_none() {
                    break
                } else {
                    labels.push(component_name.to_owned());
                }
            } else {
                return Err(Box::new(Error::InvalidPathComponent));
            }
        }

        let file_stem = rel_file_path.file_stem().ok_or(Error::InvalidPathComponent)?
            .to_str().ok_or(Error::PathContainNonUnicode)?;
        let post = if is_post {
            let file_parts = file_stem.split('-').collect::<Vec<_>>();
            let date_part = file_parts[0..3].join("-");
            let label_part = file_parts[..3].join("-");
            Some(DocumentPostLabel {
                date: date_part,
                label: label_part,
            })
        } else {
            if file_stem != "index" {
                labels.push(file_stem.to_owned());
            }
            None
        };

        let document_name = DocumentName {
            labels,
            post
        };

        Ok(Document {
            site,
            name: document_name,
            source_path: file_path.to_owned(),
            typ,
        })
    }
}
