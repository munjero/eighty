mod asciidoc;
mod markdown;

use crate::{site::SiteMetadata, Error};
use std::{
    fmt,
    path::{Component, Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use tera::Tera;

#[derive(Hash, Eq, Clone, PartialEq, Debug)]
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
pub struct DocumentMetadata {
    pub site: Arc<SiteMetadata>,
    pub name: DocumentName,
    pub typ: DocumentType,
    pub modified: SystemTime,
    pub source_path: PathBuf,
}

impl DocumentMetadata {
    pub fn new(
        site: Arc<SiteMetadata>,
        file_path: &Path,
        typ: DocumentType,
        modified: SystemTime,
    ) -> Result<DocumentMetadata, Error> {
        let rel_file_path = file_path.strip_prefix(&site.path)?;
        let name = derive_name(&rel_file_path)?;

        Ok(DocumentMetadata {
            site,
            name,
            source_path: file_path.to_owned(),
            modified,
            typ,
        })
    }

    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::new();

        for label in &self.name.labels {
            path.push(label);
        }

        if let Some(post) = self.name.post.as_ref() {
            path.push(post.date.split('-').collect::<Vec<_>>().join("/"));
            path.push(&post.label);
        }

        path.push("index.html");

        path
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
pub struct RenderedDocument {
    pub metadata: Arc<DocumentMetadata>,
    pub title: String,
    pub content: String,
}

impl RenderedDocument {
    pub fn new(document: Arc<DocumentMetadata>) -> Result<RenderedDocument, Error> {
        println!(
            "[{}] Rendering document {} ...",
            document.site.name, document.name
        );

        let rel_file_path = document
            .source_path
            .strip_prefix(&document.site.path)?;

        Ok(match document.typ {
            DocumentType::AsciiDoc => {
                let output = self::asciidoc::process_asciidoc(&document.site.path, &rel_file_path)?;

                RenderedDocument {
                    metadata: document,
                    title: output.document.title,
                    content: output.document.content,
                }
            }
            DocumentType::Markdown => {
                let output = self::markdown::process_markdown(&document.site.path, &rel_file_path)?;

                RenderedDocument {
                    metadata: document,
                    title: output.title,
                    content: output.content,
                }
            }
        })
    }
}
