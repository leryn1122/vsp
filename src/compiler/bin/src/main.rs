use std::collections::HashMap;

use clap::ArgMatches;
use clap::Command;
use vsp_bin::REPORT_URL;
use vsp_support::debug_println;
use vsp_support::exitcode::EXIT_FAILURE;
use vsp_support::resources_str;

pub(crate) mod ops;

pub type CommandLineHandler = fn(&ArgMatches) -> anyhow::Result<()>;

/// All the command line entrypoints.
fn main() {
  let mut command = create_command_line_runner();
  command.execute().unwrap_or_else(exit_with_error);
}

fn exit_with_error(error: anyhow::Error) {
  eprintln!("{}", error);
  std::process::exit(EXIT_FAILURE);
}

/// Create a command line runner instance.
/// TODO: Simplify using macros
pub(crate) fn create_command_line_runner<'ctx>() -> CommandLineRunner<'ctx> {
  let mut handlers: HashMap<&str, CommandLineHandler> = HashMap::new();
  let command = Command::new(env!("CARGO_BIN_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand_value_name("TOOLCHAIN")
    .subcommand_help_heading("Toolchains")
    .arg_required_else_help(true)
    .propagate_version(true)
    .help_expected(true)
    .multicall(true)
    .after_help(format!(resources_str!("help.txt"), url = REPORT_URL));

  handlers.insert("clean", crate::ops::clean::entrypoint);
  handlers.insert("compile", crate::ops::compile::entrypoint);
  handlers.insert("completion", crate::ops::completion::entrypoint);
  handlers.insert("debug", crate::ops::debug::entrypoint);
  handlers.insert("dump", crate::ops::dump::entrypoint);
  handlers.insert("lsp", crate::ops::lsp::entrypoint);
  handlers.insert("new", crate::ops::new::entrypoint);
  handlers.insert("pm", crate::ops::pm::entrypoint);
  handlers.insert("repl", crate::ops::repl::entrypoint);
  handlers.insert("test", crate::ops::test::entrypoint);

  handlers.insert("vspc", crate::ops::new::entrypoint);
  handlers.insert("vspdp", crate::ops::dump::entrypoint);
  handlers.insert("vspdb", crate::ops::debug::entrypoint);
  handlers.insert("vsplsp", crate::ops::lsp::entrypoint);
  handlers.insert("vspm", crate::ops::pm::entrypoint);
  handlers.insert("vspsh", crate::ops::repl::entrypoint);

  CommandLineRunner {
    command: command
      .clone()
      .subcommand(
        command
          .subcommand_required(true)
          .arg_required_else_help(true)
          .disable_help_subcommand(false)
          .disable_help_flag(false)
          .disable_version_flag(false)
          .subcommands(&[
            crate::ops::clean::cli(false),
            crate::ops::compile::cli(false),
            crate::ops::completion::cli(false),
            crate::ops::debug::cli(false),
            crate::ops::dump::cli(false),
            crate::ops::lsp::cli(false),
            crate::ops::new::cli(false),
            crate::ops::pm::cli(false),
            crate::ops::repl::cli(false),
            crate::ops::test::cli(false),
          ]),
      )
      .subcommands(&[
        crate::ops::compile::cli(true),
        crate::ops::debug::cli(true),
        crate::ops::dump::cli(true),
        crate::ops::lsp::cli(true),
        crate::ops::pm::cli(true),
        crate::ops::repl::cli(true),
      ]),
    handlers,
  }
}

pub struct CommandLineRunner<'ctx> {
  pub(crate) command:  Command,
  pub(crate) handlers: HashMap<&'ctx str, CommandLineHandler>,
}

impl<'ctx> CommandLineRunner<'ctx> {
  pub fn execute(&mut self) -> anyhow::Result<()> {
    let matches = self.command.get_matches_mut();
    let subcommand = matches.subcommand();
    match subcommand {
      Some((env!("CARGO_BIN_NAME"), args)) => {
        let cmd = args.subcommand_name();
        debug_println!("Actual command with multicall: {:?}", cmd);
        let handler = self.handlers.get(cmd.unwrap()).unwrap();
        handler(&args.clone())
      }
      Some((cmd, args)) => {
        let handler = self.handlers.get(cmd).unwrap();
        handler(&args.clone())
      }
      None => unreachable!("Unsupported commands."),
    }
  }
}
// #[macro_use]
// mod __private {
//   #[macro_export]
//   macro_rules! register_command {
//     ($command:ident $name:ident) => {
//       use crate::$name;
//       $command.register(
//         stringify!($name),
//         None,
//         $name::cli(false),
//         $name::entrypoint,
//       );
//     };
//     ($command:ident $name:ident $alias:ident) => {
//       use crate::$name;
//       $command.register(
//         stringify!($name),
//         Some($alias),
//         $name::cli(false),
//         $name::entrypoint,
//       );
//     };
//   }
// }
