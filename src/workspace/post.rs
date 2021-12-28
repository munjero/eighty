use std::collections::HashMap;
use crate::Error;
use std::path::PathBuf;
use crate::site::SiteName;
use super::FullWorkspace;
use std::ops::Deref;
use std::path::Path;

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
                post_site.insert(document_name.path(), document.content.as_bytes().to_owned());
            }

            sites.insert(site_name.clone(), SimplePostSite {
                base_url: full_site.site.config.base_url.clone(),
                files: post_site
            });
        }

        let mut spec_site_files = HashMap::new();
        spec_site_files.insert(Path::new("index.html").to_owned(), full.spec_site.index_content.as_bytes().to_owned());
        for (_, spec) in &full.spec_site.specs {
            spec_site_files.insert(spec.path.clone(), spec.redirect_content.as_bytes().to_owned());
        }
        sites.insert(SiteName("specs".into()), SimplePostSite {
            base_url: "/".to_string(),
            files: spec_site_files,
        });

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
    pub base_url: String,
    pub files: HashMap<PathBuf, Vec<u8>>,
}
