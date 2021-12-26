use crate::file::FileStore;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Hash, Eq, Clone, PartialEq, Debug)]
pub struct SiteName(String);

impl fmt::Display for SiteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfig {
    pub title: String,
    pub url: String,
    pub base_url: String,
    pub author: String,
    pub email: String,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfigLink {
    pub name: String,
    pub url: String,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfigSitemap {
    pub enable: bool,
    pub depth: Option<usize>,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Site {
    pub name: SiteName,
    pub path: PathBuf,
    pub config: SiteConfig,
}

impl Site {
    pub fn new(name: SiteName, path: &Path) -> Result<Site, Box<dyn std::error::Error>> {
        let site_config_path = path.join("_site.json");
        let site_config = serde_json::from_reader(BufReader::new(File::open(site_config_path)?))?;

        let site = Site {
            name: name.clone(),
            path: path.to_owned(),
            config: site_config,
        };

        Ok(site)
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SiteStore(HashMap<SiteName, Arc<Site>>);

impl SiteStore {
    pub fn new(root_path: &Path) -> Result<SiteStore, Box<dyn std::error::Error>> {
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

            let site = Site::new(SiteName(site_name.clone()), &site_folder.path())?;

            sites.insert(SiteName(site_name), Arc::new(site));
        }

        Ok(SiteStore(sites))
    }

    pub fn sites(&self) -> impl Iterator<Item=&Arc<Site>> {
        self.0.values()
    }
}
