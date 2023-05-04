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

/// Register all subcommand definitions and handlers to command line runner.
fn register_commands(command: &mut CommandLineRunner) {
  register_command!(command clean);
  register_command!(command compile);
  register_command!(command completion);
  register_command!(command dump);
  register_command!(command new);
  register_command!(command repl);
  register_command!(command test);
}

#[macro_use]
mod __private {
  #[macro_export]
  macro_rules! register_command {
    ($command:ident $name:ident $(,)?) => {
      use crate::ops::$name;
      $command.register($name::cli(), $name::entrypoint);
    };
  }
}
