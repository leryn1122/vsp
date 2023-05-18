use std::path::PathBuf;

use clap::arg;
use clap::Args;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Input file path
  #[arg(short, long, required = true)]
  input:      Option<PathBuf>,
  /// Input file path
  #[arg(short, long)]
  output:     Option<PathBuf>,
  /// Print preprocessed source codes
  #[arg(long, group = "dump-type")]
  preprocess: bool,
  /// Print AST (Abstract Syntax Tree)
  #[arg(short, long, group = "dump-type")]
  ast:        bool,
  /// Print LLVM IR (Intermediate Representation)
  #[arg(short, long, group = "dump-type")]
  llvm:       bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> anyhow::Result<()> {
    Ok(())
  }
}
