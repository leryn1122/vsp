use std::borrow::Cow;
use std::path::PathBuf;

use clap::arg;
use clap::value_parser;
use clap::Args;
use target_lexicon::Triple;
use vsp_compiler::option::TargetOptions;
use vsp_error::VspResult;
use vsp_support::clap_ext::TripleValueParser;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Source codes to compile
  #[arg()]
  source:       PathBuf,
  /// Build only the project's binaries
  #[arg(long = "bin")]
  binary:       Option<Cow<'static, str>>,
  /// Build only the project's library
  #[arg(long = "lib")]
  library:      bool,
  /// Target triple to compile the artifacts for
  #[arg(long, value_parser = TripleValueParser::default())]
  target:       Option<Triple>,
  /// Optimization level
  #[arg(short = 'O', long, default_value_t = 3, value_parser = value_parser!(u8).range(0..=3))]
  optimization: u8,
  /// Enable verbose mode
  #[arg(short, long)]
  verbose:      bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    let filename = &self.source;
    let target_options = self.new_target_options();

    vsp_compiler::start_compile(&filename, target_options)
  }
}

impl CandidateArgument {
  pub fn new_target_options(&self) -> TargetOptions {
    let mut target_options = TargetOptions::default();
    if let Some(target) = &self.target {
      target_options.set_target_triple(target.clone());
    }
    target_options.set_binary(self.binary.clone());
    target_options.set_library(self.library);
    target_options.set_optimization(self.optimization);
    target_options
  }
}
