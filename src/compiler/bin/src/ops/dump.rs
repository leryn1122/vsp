use std::path::PathBuf;

use clap::arg;
use clap::Args;
use vsp_dump::create_dumper;
use vsp_dump::DumpType;
use vsp_error::VspError;
use vsp_error::VspResult;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Input file path
  #[arg(short, long, required = true)]
  input:      PathBuf,
  // /// Output file path
  // #[arg(short, long)]
  // output:     Option<PathBuf>,
  /// Print token stream
  #[arg(long, group = "dump-type")]
  token:      bool,
  /// Print preprocessed source codes
  #[arg(short = 'P', long, group = "dump-type", visible_aliases = ["pp"])]
  preprocess: bool,
  /// Print AST (Abstract syntax tree)
  #[arg(short = 'A', long, group = "dump-type")]
  ast:        bool,
  /// Print LLVM IR (Intermediate representation)
  #[arg(long, group = "dump-type")]
  llvm:       bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    let dump_type = DumpType::from(
      [self.token, self.preprocess, self.ast, self.llvm]
        .into_iter()
        .position(|b| b)
        .ok_or(VspError::new(
          "Any of dump type is required. See details using `--help`.",
        ))
        .map(|t| t as u8)?,
    );

    let mut dumper = create_dumper(dump_type);
    dumper.from(&self.input.clone());
    dumper.dump()
  }
}
