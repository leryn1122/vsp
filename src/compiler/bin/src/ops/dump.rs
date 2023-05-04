use std::path::PathBuf;

use clap::arg;
use clap::value_parser;
use clap::ArgGroup;
use clap::ArgMatches;
use clap::Command;

pub(crate) fn cli() -> Command {
  Command::new("dump")
    .about("Dump tools for miscellaneous utilities on source codes.")
    .arg_required_else_help(true)
    .args(&[
      arg!(-i --input [path] "Input file path")
        .required(true)
        .value_parser(value_parser!(PathBuf)),
      arg!(-o --output [path] "Output file path")
        .required(false)
        .value_parser(value_parser!(PathBuf)),
      arg!(--ast "Print AST (Abstract Syntax Tree)"),
      arg!(--llvm-ir "Print LLVM IR (Intermediate Representation)"),
    ])
    .group(ArgGroup::new("type").args(["ast", "llvm-ir"]).required(true))
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  let ast = args.get_flag("ast");
  let input = args.get_one::<String>("input").unwrap();
  Ok(())
}
