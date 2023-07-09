use clap::arg;
use clap::Args;

use vsp_error::VspResult;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Run all unit tests only
  #[arg(short, long)]
  unittest: bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    Ok(())
  }
}
