use super::{MetadatadSite, MetadatadWorkspace};
use crate::{
    document::{DocumentName, RenderedDocument},
    file::FileMetadata,
    site::{SiteMetadata, SiteName},
    Error,
};
use rayon::prelude::*;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

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
            .map(|(name, site)| Ok((name.clone(), RenderedSite::new(&site)?)))
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            sites,
            root_path: metadata.root_path.clone(),
        })
    }

    pub fn new_with_old(metadata: &MetadatadWorkspace, old: &RenderedWorkspace) -> Result<RenderedWorkspace, Error> {
        let sites = metadata
            .sites
            .par_iter()
            .map(|(name, site)| {
                if let Some(old_site) = old.sites.get(&name) {
                    Ok((name.clone(), RenderedSite::new_with_old(&site, old_site)?))
                } else {
                    Ok((name.clone(), RenderedSite::new(&site)?))
                }
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            sites,
            root_path: metadata.root_path.clone(),
        })
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

    pub fn new_with_old(metadata: &MetadatadSite, old: &RenderedSite) -> Result<RenderedSite, Error> {
        let documents = metadata
            .documents
            .par_iter()
            .map(|(name, document)| {
                if let Some(old_document) = old.documents.get(&name) {
                    if old_document.site_metadata == metadata.site && old_document.metadata == *document {
                        return Ok((name.clone(), old_document.clone()))
                    }
                }

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
