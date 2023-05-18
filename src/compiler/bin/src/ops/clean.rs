use std::path::PathBuf;

use anyhow::anyhow;
use clap::arg;
use clap::Args;
use target_lexicon::Triple;
use vsp_support::clap_ext::TripleValueParser;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// All things in given directory would be deleted immediately
  #[arg(long, default_value = "target")]
  path:    Option<PathBuf>,
  /// Package to clean artifacts for
  #[arg(long)]
  package: Option<String>,
  /// Target triple to clean up
  #[arg(long, value_parser = TripleValueParser::default())]
  target:  Option<Triple>,
  /// Enable verbose mode
  #[arg(short, long)]
  verbose: bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> anyhow::Result<()> {
    let target_dir = std::env::current_dir().unwrap().join("target");
    match std::fs::remove_dir_all(target_dir) {
      Ok(_) => Ok(()),
      Err(e) => Err(anyhow!(e)),
    }
  }
}
