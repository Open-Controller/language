/*
Copyright (C) 2022 PJTSearch

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod OpenControllerLib;
mod parser;

use anyhow::{Context, Result};
use log::{debug, LevelFilter};
use parser::parse_module;
use protobuf::Message;
use std::{fs, path::PathBuf, str::FromStr};
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
    #[structopt(
        short = "v",
        help = "Sets the level of verbosity",
        default_value = "INFO"
    )]
    verbosity: String,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();
    env_logger::builder()
        .filter_level(LevelFilter::from_str(&opts.verbosity)?)
        .init();
    debug!(
        "{:#?}",
        parse_module(
            &opts.input.canonicalize()?,
            &opts
                .input
                .parent()
                .context("No parent folder")?
                .to_path_buf()
        )?
    );
    fs::write(
        &opts.output,
        parse_module(
            &opts.input.canonicalize()?,
            &opts
                .input
                .parent()
                .context("No parent folder")?
                .to_path_buf(),
        )?
        .write_to_bytes()
        .context("Couldn't convert to bytes")?,
    )
    .context("Failed to write")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use protobuf::Message;
    use std::{fs, process::Command, thread::sleep, time::Duration};

    use crate::OpenControllerLib::Module;

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("language")?;

        cmd.arg("./test/file/doesnt/exist");
        cmd.arg("./output.ocbin");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("No such file or directory"));

        Ok(())
    }

    #[test]
    fn compiles_to_correct_data() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("language")?;

        cmd.arg("./test/example/home.ocdef");
        cmd.arg("/tmp/output.ocbin");
        let mut child = cmd.spawn()?;

        sleep(Duration::from_millis(700));

        let bytes = Module::parse_from_bytes(&fs::read("/tmp/output.ocbin")?)?;
        child.kill()?;

        // println!("{}", bytes);

        assert_eq!(
            bytes,
            Module::parse_from_bytes(&fs::read("./test/example/expected.ocbin")?)?
        );
        // println!("{}", fs::read("./test/example/expected.ocbin")?);

        Ok(())
    }
}
