extern crate pest;
#[macro_use]
extern crate pest_derive;

mod OpenControllerLib;
mod parser;

use std::{fs, path::PathBuf, str::FromStr};
use anyhow::{Context, Result};
use log::{debug, LevelFilter};
use parser::parse_module;
use protobuf::Message;
use relative_path::RelativePathBuf;
use structopt::StructOpt;

/// CLI options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "OpenController definition compiler",
    version = "0.1.0",
    about = "Compiles ocdef files.",
    author = "PJTSearch <pjtsignups@gmail.com>"
)]
struct Opts {
    /// The input file to use
    #[structopt(parse(from_os_str), help = "Sets the input file to use")]
    input: PathBuf,

    /// The output file to use
    #[structopt(parse(from_os_str), help = "Sets the output file to use")]
    output: PathBuf,

    /// The level of verbosity
    #[structopt(short = "v", help = "Sets the level of verbosity", default_value = "INFO")]
    verbosity: String,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();
    env_logger::builder()
        .filter_level(LevelFilter::from_str(&opts.verbosity)?)
        .init();
    debug!("{:#?}", parse_module(&opts.input.canonicalize()?)?);
    fs::write(
        &opts.output,
        parse_module(&opts.input.canonicalize()?)?
            .write_to_bytes()
            .context("Couldn't convert to bytes")?,
    )
    .context("Failed to write")?;
    Ok(())
}
