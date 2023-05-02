use vsp_bin::register_command;
use vsp_support::exitcode::EXIT_FAILURE;

use crate::cli::create_command_line_runner;
use crate::cli::CommandLineRunner;

pub mod cli;
pub mod ops;

/// All the command line entries point.
fn main() {
  let mut command = create_command_line_runner();
  register_commands(&mut command);
  command.execute().unwrap_or_else(|e| exit_with_error(e));
}

fn exit_with_error(error: anyhow::Error) {
  eprintln!("{}", error);
  std::process::exit(EXIT_FAILURE);
}

fn register_commands(command: &mut CommandLineRunner) {
  register_command!(command clean);
  register_command!(command compile);
  register_command!(command completion);
  register_command!(command new);
  register_command!(command repl);
  register_command!(command test);
}
