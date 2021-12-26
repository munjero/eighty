use crate::site::SiteName;

pub struct Document {
    pub site: SiteName,
    pub name: DocumentName,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct DocumentName {
    pub labels: Vec<String>,
    pub post: Option<DocumentPostLabel>,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct DocumentPostLabel {
    pub date: String,
    pub label: String,
}
