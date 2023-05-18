use std::path::PathBuf;

use clap::arg;
use clap::Args;
use target_lexicon::Triple;
use vsp_compiler::compile;
use vsp_support::clap_ext::TripleValueParser;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Source codes to compile
  #[arg()]
  source:  PathBuf,
  /// Build only the project's binaries
  #[arg(long)]
  bin:     Option<String>,
  /// Build only the project's library
  #[arg(long)]
  lib:     bool,
  /// Target triple to compile the artifacts for
  #[arg(long, value_parser = TripleValueParser::default())]
  target:  Option<Triple>,
  /// Enable verbose mode
  #[arg(short, long)]
  verbose: bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> anyhow::Result<()> {
    let result = compile();
    Ok(())
  }
}
