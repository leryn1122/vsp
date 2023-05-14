use std::path::PathBuf;

use clap::arg;
use clap::value_parser;
use clap::ArgGroup;
use clap::ArgMatches;
use clap::Command;

pub(crate) fn cli(alias: bool) -> Command {
  let command = if alias {
    Command::new("vspdp")
  } else {
    Command::new("dump")
  };

  command
    .about("Dump tools for miscellaneous utilities on source codes.")
    .arg_required_else_help(true)
    .alias("dump123")
    .args(&[
      arg!(-i --input <path> "Input file path")
        .required(true)
        .value_parser(value_parser!(PathBuf)),
      arg!(-o --output <path> "Output file path")
        .required(false)
        .value_parser(value_parser!(PathBuf)),
      arg!(--preprocess "Print preprocessed source codes").visible_alias("--pp"),
      arg!(--ast "Print AST (Abstract Syntax Tree)"),
      arg!(--llvm "Print LLVM IR (Intermediate Representation)"),
    ])
    .group(ArgGroup::new("type").args(["ast", "llvm", "preprocess"]).required(true))
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  let input = args.get_one::<PathBuf>("input").unwrap();
  let output = args.get_one::<PathBuf>("output").unwrap();

  let ast = args.get_flag("ast");
  let llvm = args.get_flag("llvm");
  let preprocess = args.get_flag("preprocess");

  Ok(())
}
