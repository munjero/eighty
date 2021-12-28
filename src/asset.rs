use crate::{
    document::{DocumentMetadata, DocumentName, DocumentType, RenderedDocument},
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
