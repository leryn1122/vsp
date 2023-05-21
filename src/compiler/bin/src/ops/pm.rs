use clap::Args;
use vsp_error::VspResult;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> VspResult<()> {
    Ok(())
  }
}
