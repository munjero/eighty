use std::collections::HashMap;
use crate::Error;
use std::path::PathBuf;
use crate::site::SiteName;
use super::FullWorkspace;
use std::ops::Deref;
use std::sync::Arc;
use crate::site::SiteMetadata;

pub struct SimplePostWorkspace(pub HashMap<SiteName, SimplePostSite>);

impl SimplePostWorkspace {
    pub fn new(full: &FullWorkspace) -> Result<Self, Error> {
        let mut sites = HashMap::new();

        for (site_name, full_site) in &full.sites {
            let mut post_site = HashMap::new();

            for (asset_path, asset) in &full.assets.assets {
                post_site.insert(asset_path.clone(), asset.clone());
            }

            for (file_path, file) in full_site.files.as_ref() {
                post_site.insert(file_path.clone(), file.content.clone());
            }

            for (document_name, document) in &full_site.documents {
                post_site.insert(document_name.path(), document.layouted.as_bytes().to_owned());
            }

            sites.insert(site_name.clone(), SimplePostSite {
                site: full_site.site.clone(),
                files: post_site
            });
        }

        Ok(Self(sites))
    }
}

impl Deref for SimplePostWorkspace {
    type Target = HashMap<SiteName, SimplePostSite>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SimplePostSite {
    pub site: Arc<SiteMetadata>,
    pub files: HashMap<PathBuf, Vec<u8>>,
}
