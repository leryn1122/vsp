use clap::ArgMatches;
use clap::Command;

use crate::Config;

pub(crate) fn cli() -> Command {
  Command::new("run")
    .about("Language runtime launcher")
    .arg_required_else_help(true)
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &ArgMatches) -> anyhow::Result<()> {
  todo!()
}
