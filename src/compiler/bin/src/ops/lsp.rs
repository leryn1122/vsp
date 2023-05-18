use clap::arg;
use clap::Args;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Run as a daemon
  #[arg(short, long)]
  daemon:              bool,
  /// Print configuration schema
  #[arg(short, long)]
  print_config_schema: bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> anyhow::Result<()> {
    Ok(())
  }
}
