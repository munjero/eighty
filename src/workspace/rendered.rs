use crate::{
    document::{DocumentMetadata, DocumentName, DocumentType, RenderedDocument, LayoutedDocument},
    file::FileMetadata,
    site::{SiteMetadata, SiteName},
    sitemap::{Sitemap, LocalSitemap},
    Error,
};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use walkdir::WalkDir;
use handlebars::Handlebars;
use std::ops::Deref;
use super::{MetadatadWorkspace, MetadatadSite};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedWorkspace(pub HashMap<SiteName, RenderedSite>);

impl RenderedWorkspace {
    pub fn new(metadata: &MetadatadWorkspace) -> Result<RenderedWorkspace, Error> {
        let documents = metadata
            .par_iter()
            .map(|(name, site)| {
                Ok((
                    name.clone(),
                    RenderedSite::new(&site)?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self(documents))
    }
}

impl Deref for RenderedWorkspace {
    type Target = HashMap<SiteName, RenderedSite>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
