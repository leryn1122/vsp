use std::collections::HashMap;

use clap::ArgMatches;
use clap::Command;
use vsp_bin::REPORT_URL;
use vsp_support::resources_str;

pub type CommandLineHandler = fn(&ArgMatches) -> anyhow::Result<()>;

pub struct CommandLineRunner {
  pub(crate) command:  Command,
  pub(crate) handlers: HashMap<String, CommandLineHandler>,
}

/// Create a command line runner instance.
pub(crate) fn create_command_line_runner() -> CommandLineRunner {
  let command = Command::new(env!("CARGO_BIN_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand_value_name("TOOLCHAIN")
    .subcommand_help_heading("Toolchains")
    .allow_external_subcommands(true)
    .arg_required_else_help(true)
    .propagate_version(true)
    .help_expected(true)
    .after_help(format!(resources_str!("help.txt"), url = REPORT_URL));

  CommandLineRunner {
    command:  command,
    handlers: HashMap::new(),
  }
}

impl CommandLineRunner {
  pub fn execute(&mut self) -> anyhow::Result<()> {
    let matches = self.command.get_matches_mut();
    let subcommand = matches.subcommand();
    match subcommand {
      Some((cmd, args)) => {
        let handler = self.handlers.get(cmd).unwrap();
        handler(&args.clone())
      }
      None => unreachable!("Unsupported commands."),
    }
  }

  /// TODO: This method need to be optimized to drop too many clones.
  pub fn register(&mut self, subcommand: Command, handler: CommandLineHandler) {
    let binding = subcommand.clone();
    let name = binding.get_name().clone();
    let command = &self.command.clone().subcommand(subcommand);
    self.command = command.clone();
    self.handlers.insert(name.to_string(), handler);
  }
}
