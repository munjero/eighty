use std::path::PathBuf;
use crate::site::SiteName;

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Document {
    pub site: SiteName,
    pub name: DocumentName,
    pub path: PathBuf,
}

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
