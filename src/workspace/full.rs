use crate::{
    document::{DocumentMetadata, DocumentName},
    file::FileMetadata,
    site::{SiteMetadata, SiteName},
    sitemap::{Sitemap, LocalSitemap},
    Error,
};
use std::{
    collections::HashMap,
    path::{PathBuf},
    sync::Arc,
};
use handlebars::Handlebars;
use crate::workspace::{RenderedWorkspace, RenderedSite};
use crate::asset::AssetStore;
use crate::document::{self, RenderedData};

pub struct FullWorkspace {
    pub root_path: PathBuf,
    pub assets: AssetStore,
    pub sites: HashMap<SiteName, FullSite>,
}

impl FullWorkspace {
    pub fn new(rendered: &RenderedWorkspace) -> Result<Self, Error> {
        let assets = AssetStore::new(&rendered.root_path)?;

        let sites = rendered
            .sites
            .iter()
            .map(|(name, site)| {
                Ok((
                    name.clone(),
                    FullSite::new(&site, &assets.handlebars)?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            root_path: rendered.root_path.clone(),
            assets,
            sites,
        })
    }
}

pub struct FullSite {
    pub site: Arc<SiteMetadata>,
    pub documents: HashMap<DocumentName, FullDocument>,
    pub files: Arc<HashMap<PathBuf, FileMetadata>>,
    pub sitemap: Sitemap,
}

impl FullSite {
    pub fn new(rendered: &RenderedSite, handlebars: &Handlebars) -> Result<Self, Error> {
        let name_titles = rendered
            .documents
            .iter()
            .map(|(k, v)| (k.clone(), v.data.title.clone()))
            .collect::<Vec<_>>();

        let sitemap = Sitemap::from(name_titles.clone());

        let full_documents = rendered
            .documents
            .iter()
            .map(|(k, v)| {
                Ok((k.clone(), FullDocument {
                    site_metadata: v.site_metadata.clone(),
                    metadata: v.metadata.clone(),
                    rendered: v.data.clone(),
                    layouted: document::layout(&v, &sitemap, handlebars)?,
                    local_sitemap: sitemap.local(&k).ok_or(Error::DocumentNotFound)?,
                }))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self {
            site: rendered.site.clone(),
            documents: full_documents,
            files: rendered.files.clone(),
            sitemap,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct FullDocument {
    pub site_metadata: Arc<SiteMetadata>,
    pub metadata: Arc<DocumentMetadata>,
    pub rendered: Arc<RenderedData>,
    pub layouted: String,
    pub local_sitemap: LocalSitemap,
}
