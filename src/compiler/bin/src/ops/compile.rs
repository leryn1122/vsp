use clap::ArgMatches;
use clap::Command;

use crate::CommandLine;

pub(crate) fn cli() -> Command {
  Command::new("compile")
    .about("Language compiler")
    .arg_required_else_help(true)
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut CommandLine, args: &ArgMatches) -> anyhow::Result<()> {
  Ok(())
}
