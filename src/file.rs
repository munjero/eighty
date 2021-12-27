use crate::site::SiteMetadata;
use std::{path::PathBuf, sync::Arc, time::SystemTime};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FileMetadata {
    pub site: Arc<SiteMetadata>,
    pub path: PathBuf,
    pub source_path: PathBuf,
    pub modified: SystemTime,
    pub content: Vec<u8>,
}
