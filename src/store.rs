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

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SiteMetadataStore {
    pub sites: HashMap<SiteName, Arc<SiteMetadataStoreItem>>,
}

impl SiteMetadataStore {
    pub fn new(root_path: &Path) -> Result<SiteMetadataStore, Error> {
        let mut sites = HashMap::new();

        let root_subfolders = fs::read_dir(root_path)?;

        for site_folder in root_subfolders {
            let site_folder = site_folder?;
            let site_name = site_folder
                .file_name()
                .into_string()
                .map_err(|_| Error::PathContainNonUnicode)?;

            if site_name.starts_with(".") || site_name.starts_with("_") {
                continue;
            }

            println!("[{}] Generating the data for site ...", site_name);

            let site = Arc::new(SiteMetadata::new(
                SiteName(site_name.clone()),
                &site_folder.path(),
            )?);
            let item = Arc::new(SiteMetadataStoreItem::new(site.clone())?);

            sites.insert(SiteName(site_name), item);
        }

        Ok(SiteMetadataStore { sites })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SiteMetadataStoreItem {
    pub site: Arc<SiteMetadata>,
    pub documents: HashMap<DocumentName, Arc<DocumentMetadata>>,
    pub files: HashMap<PathBuf, Arc<FileMetadata>>,
}

impl SiteMetadataStoreItem {
    pub fn new(
        site: Arc<SiteMetadata>,
    ) -> Result<SiteMetadataStoreItem, Error> {
        let mut documents = HashMap::new();
        let mut files = HashMap::new();

        let walker = WalkDir::new(&site.path).into_iter().filter_entry(|entry| {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name == "_posts" && entry.file_type().is_dir() {
                    return true;
                }

                if file_name.starts_with(".") || file_name.starts_with("_") {
                    return false;
                }

                return true;
            }

            return false;
        });

        for entry in walker {
            let entry = entry?;

            if entry.file_type().is_file() {
                let modified = fs::metadata(entry.path())?.modified()?;

                let typ = if let Some(extension) = entry.path().extension() {
                    let extension = extension.to_str().ok_or(Error::PathContainNonUnicode)?;

                    match extension {
                        "md" => Some(DocumentType::Markdown),
                        "adoc" => Some(DocumentType::AsciiDoc),
                        _ => None,
                    }
                } else {
                    None
                };

                if let Some(typ) = typ {
                    let document =
                        DocumentMetadata::new(site.clone(), entry.path(), typ, modified)?;
                    documents.insert(document.name.clone(), Arc::new(document));
                } else {
                    let rel_file_path = entry.path().strip_prefix(&site.path)?;
                    let content = fs::read(entry.path())?;

                    let file = FileMetadata {
                        site: site.clone(),
                        path: rel_file_path.to_owned(),
                        source_path: entry.path().to_owned(),
                        content,
                        modified,
                    };

                    files.insert(file.path.clone(), Arc::new(file));
                }
            }
        }

        Ok(SiteMetadataStoreItem {
            site,
            documents,
            files,
        })
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct RenderedStore {
    pub documents: HashMap<SiteName, Arc<RenderedStoreItem>>,
}

impl RenderedStore {
    pub fn new(metadata: Arc<SiteMetadataStore>) -> Result<RenderedStore, Error> {
        let documents = metadata
            .sites
            .par_iter()
            .map(|(name, site)| {
                Ok((
                    name.clone(),
                    Arc::new(RenderedStoreItem::new(site.clone())?),
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
    pub documents: HashMap<DocumentName, Arc<RenderedDocument>>,
}

impl RenderedStoreItem {
    pub fn new(metadata: Arc<SiteMetadataStoreItem>) -> Result<RenderedStoreItem, Error> {
        let documents = metadata
            .documents
            .par_iter()
            .map(|(name, document)| {
                Ok((
                    name.clone(),
                    Arc::new(RenderedDocument::new(document.clone())?),
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
    pub documents: HashMap<SiteName, Arc<LayoutedStoreItem>>,
}

impl LayoutedStore {
    pub fn new(rendered: Arc<RenderedStore>, sitemaps: Arc<SitemapStore>, assets: Arc<AssetStore>) -> Result<LayoutedStore, Error> {
        let documents = rendered
            .documents
            .iter()
            .map(|(name, site)| {
                let sitemap = sitemaps.sitemaps.get(&name).ok_or(Error::SiteNotExist)?;

                Ok((
                    name.clone(),
                    Arc::new(LayoutedStoreItem::new(site.clone(), sitemap.clone(), &assets.handlebars)?),
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
    pub documents: HashMap<DocumentName, Arc<LayoutedDocument>>,
}

impl LayoutedStoreItem {
    pub fn new(rendered: Arc<RenderedStoreItem>, sitemap: Arc<SitemapStoreItem>, handlebars: &Handlebars<'static>) -> Result<LayoutedStoreItem, Error> {
        let documents = rendered.documents
            .iter()
            .map(|(name, document)| {
                Ok((
                    name.clone(),
                    Arc::new(LayoutedDocument::new(document.clone(), &sitemap.sitemap, handlebars)?),
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
    pub sitemaps: HashMap<SiteName, Arc<SitemapStoreItem>>,
}

impl SitemapStore {
    pub fn new(rendered: Arc<RenderedStore>) -> Result<SitemapStore, Error> {
        let sitemaps = rendered
            .documents
            .iter()
            .map(|(name, site)| {
                Ok((
                    name.clone(),
                    Arc::new(SitemapStoreItem::new(site.clone())?),
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
    pub fn new(rendered: Arc<RenderedStoreItem>) -> Result<SitemapStoreItem, Error> {
        let ordered_name_titles = {
            let mut name_titles = rendered
                .documents
                .iter()
                .map(|(k, v)| (k.clone(), v.title.clone()))
                .collect::<Vec<_>>();

            name_titles.sort_by_key(|(k, _)| k.clone());
            name_titles
        };

        let mut sitemap = Sitemap { items: Vec::new() };
        for (name, title) in &ordered_name_titles {
            if !name.is_root() {
                sitemap.insert(name.clone(), title.clone());
            }
        }

        let mut local_sitemaps = HashMap::new();
        for (name, _) in &ordered_name_titles {
            if let Some(local_sitemap) = sitemap.local(&name) {
                local_sitemaps.insert(name.clone(), local_sitemap);
            }
        }

        Ok(SitemapStoreItem { sitemap, local_sitemaps })
    }
}
