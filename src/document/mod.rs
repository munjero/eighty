mod asciidoc;
mod layout;
mod markdown;

pub use self::layout::layout;

use crate::{site::SiteMetadata, Error};
use std::{
    fmt,
    path::{Component, Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

#[derive(Hash, Eq, Clone, PartialEq, Debug, PartialOrd, Ord)]
pub struct DocumentName {
    pub labels: Vec<String>,
    pub post: Option<DocumentPostLabel>,
}

impl fmt::Display for DocumentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.labels.is_empty() && self.post.is_none() {
            write!(f, "[index]")?;
            return Ok(());
        }

        write!(f, "{}", self.labels.join("/"))?;
        if let Some(post) = self.post.as_ref() {
            write!(f, "[post:{},{}]", post.date, post.label)?;
        }
        Ok(())
    }
}

impl DocumentName {
    pub fn path(&self) -> PathBuf {
        let mut path = self.folder_path();

        path.push("index.html");

        path
    }

    pub fn folder_path(&self) -> PathBuf {
        let mut path = PathBuf::new();

        for label in &self.labels {
            path.push(label);
        }

        if let Some(post) = self.post.as_ref() {
            path.push(post.date.split('-').collect::<Vec<_>>().join("/"));
            path.push(&post.label);
        }

        path
    }

    pub fn is_matched(&self, other: &Path) -> bool {
        other == self.path() || other == self.folder_path()
    }

    pub fn is_ancestor_of(&self, child: &Self) -> bool {
        if self.post.is_some() {
            return false;
        }

        if self.labels.len() > child.labels.len() {
            return false;
        }

        if self.labels[0..self.labels.len()] != child.labels[0..self.labels.len()] {
            return false;
        }

        if self.labels.len() == child.labels.len() {
            return self.post.is_none() && child.post.is_some();
        }

        return true;
    }

    pub fn is_root(&self) -> bool {
        self.post.is_none() && self.labels.is_empty()
    }
}

#[derive(Hash, Eq, Clone, PartialEq, Debug, PartialOrd, Ord)]
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
pub struct DocumentMetadata {
    pub name: DocumentName,
    pub typ: DocumentType,
    pub modified: SystemTime,
    pub source_path: PathBuf,
}

impl DocumentMetadata {
    pub fn new(
        site: &SiteMetadata,
        file_path: &Path,
        typ: DocumentType,
        modified: SystemTime,
    ) -> Result<DocumentMetadata, Error> {
        let rel_file_path = file_path.strip_prefix(&site.source_path)?;
        let name = derive_name(&rel_file_path)?;

        Ok(DocumentMetadata {
            name,
            source_path: file_path.to_owned(),
            modified,
            typ,
        })
    }
}

fn derive_name(rel_file_path: &Path) -> Result<DocumentName, Error> {
    let mut labels = Vec::new();
    let mut is_post = false;
    let mut components = rel_file_path.components().peekable();

    while let Some(component) = components.next() {
        if let Component::Normal(component_name) = component {
            let component_name = component_name
                .to_str()
                .ok_or(Error::PathContainNonUnicode)?;

            if component_name == "_posts" {
                is_post = true;
                break;
            }

            if components.peek().is_none() {
                break;
            } else {
                labels.push(component_name.to_owned());
            }
        } else {
            return Err(Error::InvalidPathComponent);
        }
    }

    let file_stem = rel_file_path
        .file_stem()
        .ok_or(Error::InvalidPathComponent)?
        .to_str()
        .ok_or(Error::PathContainNonUnicode)?;
    let post = if is_post {
        let file_parts = file_stem.split('-').collect::<Vec<_>>();
        let date_part = file_parts[0..3].join("-");
        let label_part = file_parts[3..].join("-");
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

    Ok(DocumentName { labels, post })
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedData {
    pub title: String,
    pub content: String,
    pub toc: Option<String>,
    pub description: String,
    pub description_content: Option<String>,
    pub license: Option<String>,
    pub license_code: Option<String>,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedDocument {
    pub site_metadata: Arc<SiteMetadata>,
    pub metadata: Arc<DocumentMetadata>,
    pub data: Arc<RenderedData>,
}

impl RenderedDocument {
    pub fn new(
        site: Arc<SiteMetadata>,
        document: Arc<DocumentMetadata>,
    ) -> Result<RenderedDocument, Error> {
        println!("[{}] Rendering document {} ...", site.name, document.name);

        let rel_file_path = document.source_path.strip_prefix(&site.source_path)?;

        Ok(match document.typ {
            DocumentType::AsciiDoc => {
                let output = self::asciidoc::process_asciidoc(&site.source_path, &rel_file_path)?;

                RenderedDocument {
                    site_metadata: site,
                    metadata: document,
                    data: Arc::new(RenderedData {
                        title: output.document.title,
                        content: output.document.content,
                        toc: output.document.toc,
                        description: output.document.description.clone(),
                        description_content: None,
                        license: output.document.license,
                        license_code: output.document.license_code,
                    }),
                }
            }
            DocumentType::Markdown => {
                let output = self::markdown::process_markdown(&site.source_path, &rel_file_path)?;

                RenderedDocument {
                    site_metadata: site,
                    metadata: document,
                    data: Arc::new(RenderedData {
                        title: output.title,
                        content: output.content,
                        toc: Some(output.toc),
                        description: output.description,
                        description_content: Some(output.description_content),
                        license: None,
                        license_code: None,
                    }),
                }
            }
        })
    }
}
