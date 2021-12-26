mod document;
mod file;
mod site;

use crate::site::SiteStore;
use clap::{App, Arg, SubCommand};
use std::{fmt, path::Path};

#[derive(Eq, Clone, PartialEq, Debug)]
pub enum Error {
    PathContainNonUnicode,
    InvalidPathComponent,
    RunCommandFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Eighty")
        .subcommand(
            SubCommand::with_name("build").arg(Arg::with_name("root").index(1).required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));
        let store = SiteStore::new(root_path);

        // println!("{:?}", store);
    }

    Ok(())
}
