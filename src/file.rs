use crate::{
    document::{Document, DocumentName, DocumentType},
    site::SiteMetadata,
    Error,
};
use std::{collections::HashMap, path::PathBuf, sync::Arc, fs};
use walkdir::WalkDir;

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FileMetadata {
    pub site: Arc<SiteMetadata>,
    pub path: PathBuf,
    pub source_path: PathBuf,
    pub content: Vec<u8>,
}
