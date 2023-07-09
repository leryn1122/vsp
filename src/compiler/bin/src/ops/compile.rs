use std::path::PathBuf;

use clap::arg;
use clap::Args;
use vsp_compiler::SimpleCompilerInstance;
use vsp_error::VspResult;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Source codes to compile
  #[arg()]
  source: PathBuf,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    let filename = &self.source;
    let mut compiler = SimpleCompilerInstance::new();
    compiler.compile(filename)
  }
}
