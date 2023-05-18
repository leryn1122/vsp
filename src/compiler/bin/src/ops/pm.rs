use clap::Args;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> anyhow::Result<()> {
    Ok(())
  }
}
