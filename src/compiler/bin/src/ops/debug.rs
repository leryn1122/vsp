use std::path::PathBuf;

use clap::arg;
use clap::value_parser;
use clap::Args;

use vsp_dbg::dbg::DebuggerInstance;
use vsp_error::VspResult;
use vsp_platform::proc::PID_MAX;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// The process with the given PID attached to
  #[arg(short = 'p', long, value_parser = value_parser ! (u32).range(0..PID_MAX as i64))]
  pid: Option<u32>,
  /// The process with the given name attached to
  #[arg(short = 'n', long)]
  name: Option<String>,
  /// Source file
  #[arg(short = 'S', long)]
  source: Option<PathBuf>,
  /// Tells the debugger to wait for a process with the given pid or name to launch before
  /// attaching.
  #[arg(long = "wait-for")]
  wait_for: bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    let debugger = DebuggerInstance::default();
    debugger.core_loop()
  }
}
