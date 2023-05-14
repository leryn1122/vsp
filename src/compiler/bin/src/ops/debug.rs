use std::path::PathBuf;

use clap::arg;
use clap::value_parser;
use clap::ArgMatches;
use clap::Command;
use clap::ValueHint;
use vsp_dbg::dbg::DebuggerInstance;
use vsp_platform::proc::PID_MAX;

pub(crate) fn cli(alias: bool) -> Command {
  let command = if alias {
    Command::new("vspdb")
  } else {
    Command::new("debug")
  };
  command.about("Native debugger")
    .args(&[
      arg!(-p --pid <pid> "The process with the given PID attached to")
        .value_parser(value_parser!(u32).range(0..PID_MAX as i64)),
      arg!(-n --name <name> "The process with the given name attached to"),
      arg!(-S --source <file> "Source file")
        .value_parser(value_parser!(PathBuf))
        .value_hint(ValueHint::FilePath),
      arg!(--"wait-for" "Tells the debugger to wait for a process with the given pid or name to launch before attaching."),
  ])
}

#[allow(unused_variables, unused_mut)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  let pid = args.get_one::<u32>("pid");
  let name = args.get_one::<String>("name");
  let source = args.get_one::<PathBuf>("source");
  let wait_for = args.get_flag("wait-for");

  let mut debugger = DebuggerInstance::default();
  debugger.core_loop()
}
