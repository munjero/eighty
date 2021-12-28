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
};
use walkdir::WalkDir;
use handlebars::Handlebars;
use crate::workspace::{MetadatadWorkspace, MetadatadSite, RenderedWorkspace, RenderedSite};
use crate::asset::AssetStore;

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct LayoutedStore {
    pub documents: HashMap<SiteName, LayoutedStoreItem>,
}

impl LayoutedStore {
    pub fn new(rendered: &RenderedWorkspace, sitemaps: &SitemapStore, assets: &AssetStore) -> Result<LayoutedStore, Error> {
        let documents = rendered
            .iter()
            .map(|(name, site)| {
                let sitemap = sitemaps.sitemaps.get(&name).ok_or(Error::SiteNotExist)?;

                Ok((
                    name.clone(),
                    LayoutedStoreItem::new(&site, &sitemap, &assets.handlebars)?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(LayoutedStore {
            documents,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct LayoutedStoreItem {
    pub documents: HashMap<DocumentName, LayoutedDocument>,
}

impl LayoutedStoreItem {
    pub fn new(rendered: &RenderedSite, sitemap: &SitemapStoreItem, handlebars: &Handlebars<'static>) -> Result<LayoutedStoreItem, Error> {
        let documents = rendered.documents
            .iter()
            .map(|(name, document)| {
                Ok((
                    name.clone(),
                    LayoutedDocument::new(&document, &sitemap.sitemap, handlebars)?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(LayoutedStoreItem {
            documents,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SitemapStore {
    pub sitemaps: HashMap<SiteName, SitemapStoreItem>,
}

impl SitemapStore {
    pub fn new(rendered: &RenderedWorkspace) -> Result<SitemapStore, Error> {
        let sitemaps = rendered
            .iter()
            .map(|(name, site)| {
                Ok((
                    name.clone(),
                    SitemapStoreItem::new(&site)?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(SitemapStore {
            sitemaps
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SitemapStoreItem {
    pub sitemap: Sitemap,
    pub local_sitemaps: HashMap<DocumentName, LocalSitemap>,
}

impl SitemapStoreItem {
    pub fn new(rendered: &RenderedSite) -> Result<SitemapStoreItem, Error> {
        let name_titles = rendered
            .documents
            .iter()
            .map(|(k, v)| (k.clone(), v.title.clone()))
            .collect::<Vec<_>>();

        let sitemap = Sitemap::from(name_titles.clone());

        let mut local_sitemaps = HashMap::new();
        for (name, _) in &name_titles {
            if let Some(local_sitemap) = sitemap.local(&name) {
                local_sitemaps.insert(name.clone(), local_sitemap);
            }
        }

        Ok(SitemapStoreItem { sitemap, local_sitemaps })
    }
}
