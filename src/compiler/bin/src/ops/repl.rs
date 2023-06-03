use clap::Args;
use vsp_cli::repl::do_run_repl;
use vsp_error::VspError;
use vsp_error::VspResult;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    match do_run_repl() {
      Ok(res) => Ok(res),
      Err(e) => Err(VspError::from(e)),
    }
  }
}
