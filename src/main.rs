use clap::{Arg, App, SubCommand};
use std::{fs::{self, File}, fmt, path::{Path, PathBuf}, collections::HashMap, io::BufReader};
use serde::{Serialize, Deserialize};

#[derive(Eq, Clone, PartialEq, Debug)]
pub enum Error {
    PathContainNonUnicode
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error { }

pub struct Document {

}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub id: String,
    pub path: PathBuf,
    pub config: SiteConfig,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let matches = App::new("Eighty")
        .subcommand(
            SubCommand::with_name("build")
                .arg(Arg::with_name("root").index(1).required(true))
        )
        .get_matches();

    let mut sites = HashMap::new();

    if let Some(matches) = matches.subcommand_matches("build") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));
        let root_subfolders = fs::read_dir(root_path)?;

        for site_folder in root_subfolders {
            let site_folder = site_folder?;
            let site_id = site_folder.file_name().into_string().map_err(|_| Error::PathContainNonUnicode)?;

            if site_id.starts_with(".") ||
                site_id.starts_with("_")
            {
                continue
            }

            let site_config_path = site_folder.path().join("_site.json");
            let site_config = serde_json::from_reader(BufReader::new(File::open(site_config_path)?))?;

            let site = Site {
                id: site_id,
                path: site_folder.path(),
                config: site_config,
            };

            sites.insert(site_folder.file_name(), site);
        }

        println!("{:?}", sites);
    }

    Ok(())
}
