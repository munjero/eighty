mod document;
mod file;
mod site;
mod store;
mod command;
mod error;
mod sitemap;
mod workspace;
mod asset;

pub use crate::error::Error;

use clap::{App, Arg, SubCommand};
use std::{path::Path};

fn main() -> Result<(), Error> {
    let matches = App::new("Eighty")
        .subcommand(
            SubCommand::with_name("build").arg(Arg::with_name("root").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .arg(Arg::with_name("root").index(1).required(true))
                .arg(Arg::with_name("site").index(2).required(true))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));

        command::build::build(&root_path)?;
    } else if let Some(matches) = matches.subcommand_matches("serve") {
        let root_path = Path::new(matches.value_of("root").expect("root is required"));
        let site_name = matches.value_of("site").expect("site is required");

        command::serve::serve(&root_path, &site_name)?;
    }

    Ok(())
}
