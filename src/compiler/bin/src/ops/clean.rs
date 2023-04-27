use std::path::PathBuf;

use anyhow::anyhow;
use clap::arg;
use clap::value_parser;
use clap::ArgMatches;
use clap::Command;

use crate::CommandLine;

pub(crate) fn cli() -> Command {
  Command::new("clean").about("Clean target directory").arg(
    arg!(--path [path] "All things in given directory would be deleted immediately.")
      .default_value("target")
      .value_parser(value_parser!(PathBuf)),
  )
}

/// Remove the current working directory target artifacts.
///   - target/*
#[allow(unused_variables)]
pub(crate) fn execute(config: &mut CommandLine, args: &ArgMatches) -> anyhow::Result<()> {
  let target_dir = config.cwd.join("target");
  match std::fs::remove_dir_all(target_dir) {
    Ok(_) => Ok(()),
    Err(e) => Err(anyhow!(e)),
  }
}
