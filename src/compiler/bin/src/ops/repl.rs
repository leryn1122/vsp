use anyhow::anyhow;
use clap::ArgMatches;
use clap::Command;
use vsp_cli::repl::do_run_repl;

use crate::Config;

pub(crate) fn cli() -> Command {
  Command::new("repl").about("REPL (Read-Eval-Print Loop) or shell")
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &ArgMatches) -> anyhow::Result<()> {
  match do_run_repl() {
    Ok(res) => Ok(res),
    Err(e) => Err(anyhow!(e)),
  }
}
