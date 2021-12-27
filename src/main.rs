mod document;
mod file;
mod site;
mod store;
mod command;
mod error;

pub use crate::error::Error;

use crate::store::{RenderedStore, SiteMetadataStore};
use clap::{App, Arg, SubCommand};
use std::{fmt, path::Path, sync::Arc};

fn main() -> Result<(), Error> {
    let matches = App::new("Eighty")
        .subcommand(
            SubCommand::with_name("build").arg(Arg::with_name("root").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("serve").arg(Arg::with_name("root").index(1).required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));

        command::build::build(&root_path)?;
    } else if let Some(matches) = matches.subcommand_matches("serve") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));

        command::serve::serve(&root_path)?;
    }

    Ok(())
}
