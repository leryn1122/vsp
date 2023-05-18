use anyhow::anyhow;
use clap::Args;
use vsp_cli::repl::do_run_repl;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> anyhow::Result<()> {
    match do_run_repl() {
      Ok(res) => Ok(res),
      Err(e) => Err(anyhow!(e)),
    }
  }
}
