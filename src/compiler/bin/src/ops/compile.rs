use std::path::PathBuf;

use anyhow::anyhow;
use clap::arg;
use clap::value_parser;
use clap::ArgMatches;
use clap::Command;
use clap::ValueHint;
use vsp_compiler::compile;
use vsp_support::clap_ext::TripleValueParser;

pub(crate) fn cli(alias: bool) -> Command {
  let command = if alias {
    Command::new("vspc")
  } else {
    Command::new("compile")
  };
  command.about("Language compiler").arg_required_else_help(true).args(&[
    arg!(<source>  "Source codes to compile")
      .required(true)
      .value_parser(value_parser!(PathBuf))
      .value_hint(ValueHint::AnyPath),
    arg!(--lib "Build only the project's library"),
    arg!(--bin <bin> "Build only the project's binaries"),
    arg!(-q --quiet "Enable quiet mode"),
    arg!(--target <triple> "Target triple to compile the artifacts for")
      .value_parser(TripleValueParser::default()),
  ])
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  let target_triple = args
    .get_one::<String>("target")
    .ok_or(anyhow!("Unsupported target triple"))
    .ok();
  let quiet = args.get_flag("quiet");
  let result = compile();
  Ok(result)
}
