use crate::{
    document::{DocumentName, RenderedDocument},
    file::FileMetadata,
    site::{SiteMetadata, SiteName},
    Error,
};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    path::{PathBuf},
    sync::Arc,
};
use super::{MetadatadWorkspace, MetadatadSite};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedWorkspace {
    pub root_path: PathBuf,
    pub sites: HashMap<SiteName, RenderedSite>,
}

impl RenderedWorkspace {
    pub fn new(metadata: &MetadatadWorkspace) -> Result<RenderedWorkspace, Error> {
        let sites = metadata
            .sites
            .par_iter()
            .map(|(name, site)| {
                Ok((
                    name.clone(),
                    RenderedSite::new(&site)?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self { sites, root_path: metadata.root_path.clone(), })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedSite {
    pub site: Arc<SiteMetadata>,
    pub documents: HashMap<DocumentName, RenderedDocument>,
    pub files: Arc<HashMap<PathBuf, FileMetadata>>,
}

impl RenderedSite {
    pub fn new(metadata: &MetadatadSite) -> Result<RenderedSite, Error> {
        let documents = metadata
            .documents
            .par_iter()
            .map(|(name, document)| {
                Ok((
                    name.clone(),
                    RenderedDocument::new(metadata.site.clone(), document.clone())?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            site: metadata.site.clone(),
            files: metadata.files.clone(),
            documents,
        })
    }
}
