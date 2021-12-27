mod document;
mod file;
mod site;
mod store;

use crate::document::RenderedDocument;
use crate::store::{SiteMetadataStore, RenderedStore};
use clap::{App, Arg, SubCommand};
use std::{fmt, path::Path, collections::HashMap, sync::Arc};

#[derive(Debug)]
pub enum Error {
    PathContainNonUnicode,
    InvalidPathComponent,
    RunCommandFailed,
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Eighty")
        .subcommand(
            SubCommand::with_name("build").arg(Arg::with_name("root").index(1).required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));

        let site_metadata_store = Arc::new(SiteMetadataStore::new(root_path)?);
        let rendered_store = Arc::new(RenderedStore::new(site_metadata_store.clone())?);
    }

    Ok(())
}
