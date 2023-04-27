use std::collections::HashMap;
use std::path::PathBuf;

use vsp_bin::EXIT_FAILURE;
use vsp_cli::shell::Shell;

use crate::ops::clean;
use crate::ops::compile;
use crate::ops::completion;
use crate::ops::new;
use crate::ops::repl;
use crate::ops::test;
use crate::ops::CliHandler;

pub fn run(cli: &mut CommandLine) -> anyhow::Result<()> {
  cli.execute_command()
}

pub fn exit_with_error(error: anyhow::Error) {
  eprintln!("{}", error);
  std::process::exit(EXIT_FAILURE);
}

/// Packaged configuration
#[allow(dead_code)]
pub struct CommandLine {
  /// Arguments accepted in command line.
  args:           Vec<String>,
  /// PID in form of `u32`.
  pid:            u32,
  /// Current working directory.
  pub(crate) cwd: PathBuf,
  /// Builtin `clap::Command` to parse command line arguments.
  command:        clap::Command,
  /// Environment variables as K-V pairs.
  env:            HashMap<String, String>,
  /// Handlers.
  handlers:       HashMap<String, CliHandler>,
  ///
  locked:         bool,
  /// Shell
  shell:          Shell,
  /// True if verbose mode is enabled.
  verbose:        bool,
}

impl CommandLine {
  /// Register `clap` subcommands and handlers to execute in the following
  /// stages. All subcommands are delegated to register through this method.
  /// TODO: Soon use macros to replace hard code.
  pub(crate) fn register_command() -> (clap::Command, HashMap<String, CliHandler>) {
    let mut handlers = HashMap::new();
    let mut command = clap::Command::new(env!("CARGO_BIN_NAME"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .subcommand_value_name("TOOLCHAIN")
      .subcommand_help_heading("Toolchains")
      .arg_required_else_help(true);

    // vsp clean
    command = command.subcommand(clean::cli());
    handlers.insert("clean".to_string(), clean::execute as CliHandler);

    // vsp compile
    command = command.subcommand(compile::cli());
    handlers.insert("compile".to_string(), compile::execute as CliHandler);

    // vsp completion
    command = command.subcommand(completion::cli());
    handlers.insert("completion".to_string(), completion::execute as CliHandler);

    // vsp new
    command = command.subcommand(new::cli());
    handlers.insert("new".to_string(), new::execute as CliHandler);

    // vsp repl
    command = command.subcommand(repl::cli());
    handlers.insert("repl".to_string(), repl::execute as CliHandler);

    // vsp test
    command = command.subcommand(test::cli());
    handlers.insert("test".to_string(), test::execute as CliHandler);

    (command, handlers)
  }

  pub fn execute_command(&mut self) -> anyhow::Result<()> {
    let matches = self.command.get_matches_mut();
    let subcommand = matches.subcommand();
    match subcommand {
      Some((cmd, args)) => {
        let handler = self.handlers.get(cmd).unwrap();
        handler(self, &args.clone())
      }
      None => unreachable!("Unsupported commands."),
    }
  }
}

impl Default for CommandLine {
  /// Construct default config at current directory.
  fn default() -> Self {
    let env: HashMap<_, _> = std::env::vars_os()
      .filter_map(|(k, v)| match (k.into_string(), v.into_string()) {
        (Ok(k), Ok(v)) => Some((k, v)),
        _ => None,
      })
      .collect();

    let (command, handlers) = Self::register_command();
    let shell = Shell::default();

    Self {
      args:     std::env::args().collect(),
      pid:      std::process::id(),
      cwd:      PathBuf::new(),
      command:  command,
      env:      env,
      handlers: handlers,
      locked:   false,
      shell:    shell,
      verbose:  false,
    }
  }
}
