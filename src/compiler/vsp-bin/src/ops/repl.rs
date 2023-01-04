use anyhow::anyhow;

use crate::Config;

pub(crate) fn cli() -> clap::Command {
  clap::Command::new("repl").about("REPL (Read-Eval-Print Loop) or shell")
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &clap::ArgMatches) -> anyhow::Result<()> {
  match vsp_cli::repl::do_run_repl() {
    Ok(res) => Ok(res),
    Err(e) => Err(anyhow!(e)),
  }
}
