use std::path::PathBuf;

use clap::arg;
use clap::value_parser;
use clap::ArgMatches;
use clap::Command;
use vsp_platform::proc::PID_MAX;

pub(crate) fn cli(alias: bool) -> Command {
  let command = if alias {
    Command::new("vspdb")
  } else {
    Command::new("debug")
  };
  command.about("Native debugger")
    .arg_required_else_help(true).args(&[
    arg!(-p --pid <pid> "The process with the given PID attached to.")
      .value_parser(value_parser!(u32).range(0..PID_MAX as i64)),
    arg!(-n --name <name> "The process with the given name attached to."),
    arg!(-S --source <file> "Source file")
      .value_parser(value_parser!(PathBuf)),
    arg!(--"wait-for" "Tells the debugger to wait for a process with the given pid or name to launch before attaching."),
  ])
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  Ok(())
}
