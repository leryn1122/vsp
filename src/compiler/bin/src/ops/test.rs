use clap::arg;
use clap::Args;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Run all unit tests only
  #[arg(short, long)]
  unittest: bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> anyhow::Result<()> {
    Ok(())
  }
}
