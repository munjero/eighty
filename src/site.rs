use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

#[derive(Hash, Eq, Clone, PartialEq, Debug)]
pub struct SiteName(pub String);

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
pub struct SiteMetadata {
    pub name: SiteName,
    pub path: PathBuf,
    pub config: SiteConfig,
}

impl SiteMetadata {
    pub fn new(name: SiteName, path: &Path) -> Result<SiteMetadata, Box<dyn std::error::Error>> {
        let site_config_path = path.join("_site.json");
        let site_config = serde_json::from_reader(BufReader::new(File::open(site_config_path)?))?;

        let site = SiteMetadata {
            name: name.clone(),
            path: path.to_owned(),
            config: site_config,
        };

        Ok(site)
    }
}
