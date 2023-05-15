use std::path::PathBuf;

use anyhow::anyhow;
use clap::arg;
use clap::value_parser;
use clap::ArgMatches;
use clap::Command;
use vsp_support::clap_ext::TripleValueParser;

pub(crate) fn cli(_: bool) -> Command {
  Command::new("clean").about("Clean target directory").args(&[
    arg!(--path <path> "All things in given directory would be deleted immediately")
      .default_value("target")
      .value_parser(value_parser!(PathBuf)),
    arg!(-p --package <package> "Package to clean artifacts for")
      .value_parser(value_parser!(String)),
    arg!(-q --quiet "Enable quiet mode"),
    arg!(--target <triple> "Target triple to clean").value_parser(TripleValueParser::default()),
    arg!(-v --verbose "Enable verbose mode"),
  ])
}

/// Remove the current working directory target artifacts.
///   - target/*
#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  let target_dir = std::env::current_dir().unwrap().join("target");
  // let path = args.get_one::<PathBuf>("path");
  // let quiet = args.get_flag("quiet");
  // let package = args.get_one::<String>("package");
  // let verbose = args.get_flag("verbose");
  // let triple = args.get_one::<Triple>("target");

  match std::fs::remove_dir_all(target_dir) {
    Ok(_) => Ok(()),
    Err(e) => Err(anyhow!(e)),
  }
}
