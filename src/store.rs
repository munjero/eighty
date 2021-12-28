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
use crate::workspace::{MetadatadWorkspace, MetadatadSite};

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedStore {
    pub documents: HashMap<SiteName, RenderedStoreItem>,
}

impl RenderedStore {
    pub fn new(metadata: &MetadatadWorkspace) -> Result<RenderedStore, Error> {
        let documents = metadata
            .par_iter()
            .map(|(name, site)| {
                Ok((
                    name.clone(),
                    RenderedStoreItem::new(&site)?,
                ))
            })
            .collect::<Result<_, Error>>()?;

        Ok(RenderedStore {
            documents,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedStoreItem {
    pub documents: HashMap<DocumentName, RenderedDocument>,
}

impl RenderedStoreItem {
    pub fn new(metadata: &MetadatadSite) -> Result<RenderedStoreItem, Error> {
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

        Ok(RenderedStoreItem {
            documents,
        })
    }
}

#[derive(Clone, Debug)]
pub struct AssetStore {
    pub assets: HashMap<PathBuf, Vec<u8>>,
    pub handlebars: Handlebars<'static>,
}

impl AssetStore {
    pub fn new(root_path: &Path) -> Result<AssetStore, Error> {
        let mut assets = HashMap::new();

        let asset_path = root_path.join("_assets");

        let css_path = asset_path.join("css");
        let font_path = asset_path.join("font");
        let js_path = asset_path.join("js");

        for part_path in [css_path, font_path, js_path] {
            for entry in WalkDir::new(&part_path) {
                let entry = entry?;

                if entry.file_type().is_file() {
                    let content = fs::read(entry.path())?;
                    let rel_path = entry.path().strip_prefix(&asset_path)?;

                    assets.insert(rel_path.to_owned(), content);
                }
            }
        }

        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_templates_directory(".hbs", asset_path.join("layouts"))?;

        Ok(AssetStore { assets, handlebars, })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct LayoutedStore {
    pub documents: HashMap<SiteName, LayoutedStoreItem>,
}

impl LayoutedStore {
    pub fn new(rendered: &RenderedStore, sitemaps: &SitemapStore, assets: &AssetStore) -> Result<LayoutedStore, Error> {
        let documents = rendered
            .documents
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
    pub fn new(rendered: &RenderedStoreItem, sitemap: &SitemapStoreItem, handlebars: &Handlebars<'static>) -> Result<LayoutedStoreItem, Error> {
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
    pub fn new(rendered: &RenderedStore) -> Result<SitemapStore, Error> {
        let sitemaps = rendered
            .documents
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
    pub fn new(rendered: &RenderedStoreItem) -> Result<SitemapStoreItem, Error> {
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
