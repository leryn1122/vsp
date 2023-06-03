//! The main application which is designed in a `clap_derive` pattern.
//!
//! ```bash
//! $ vsp --help
//!
//! $ vsp new helloword
//!
//! $ cd helloworld
//! $ vsp compile helloworld.vsp
//! ```
//!
//! See also the [guide](https://vsp.io/guide)
use clap::builder::Styles;
use clap::Parser;
use clap::Subcommand;
use vsp_error::VspError;
use vsp_support::exitcode;
use vsp_support::resources_str;

use crate::ops::clean;
use crate::ops::compile;
use crate::ops::completion;
use crate::ops::debug;
use crate::ops::dump;
use crate::ops::lsp;
use crate::ops::new;
use crate::ops::pm;
use crate::ops::repl;
use crate::ops::test;
use crate::ops::Entrypoint;

pub(crate) mod ops;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(subcommand_help_heading = "Toolchains")]
#[command(subcommand_value_name = "TOOLCHAINS")]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
#[command(propagate_version = true)]
#[command(disable_help_subcommand = true)]
#[command(external_subcommand = true)]
#[command(styles = get_styles())]
#[command(after_help = format!(resources_str!("help.txt"), url = vsp_bin::REPORT_URL))]
pub struct MainCommand {
  #[command(subcommand)]
  subcommand: CandidateCommand,
}

/// Registered subcommand for the application, following `clap::derive` as an enumeration.
#[derive(Subcommand)]
pub enum CandidateCommand {
  /// Clean target directory
  Clean(clean::CandidateArgument),
  /// Language compiler
  Compile(compile::CandidateArgument),
  /// Generate autocompletion scripts for the specified shell
  Completion(completion::CandidateArgument),
  /// Native debugger
  Debug(debug::CandidateArgument),
  /// Dump tools for miscellaneous utilities on source codes
  Dump(dump::CandidateArgument),
  /// Language server based on LSP (language server protocol)
  LSP(lsp::CandidateArgument),
  /// Create new project
  New(new::CandidateArgument),
  /// Project manager
  PM(pm::CandidateArgument),
  /// REPL (Read-Eval-Print Loop) or shell
  REPL(repl::CandidateArgument),
  /// Run all unit tests and integration tests
  Test(test::CandidateArgument),
}

fn main() {
  #[cfg(target_arch = "wasm32")]
  unimplemented!("Vespera on wasm32 is not supported yet.");

  let command = MainCommand::parse();
  #[rustfmt::skip]
  match command.subcommand {
    CandidateCommand::Clean     (mut args) => args.entrypoint(),
    CandidateCommand::Compile   (mut args) => args.entrypoint(),
    CandidateCommand::Completion(mut args) => args.entrypoint(),
    CandidateCommand::Debug     (mut args) => args.entrypoint(),
    CandidateCommand::Dump      (mut args) => args.entrypoint(),
    CandidateCommand::LSP       (mut args) => args.entrypoint(),
    CandidateCommand::New       (mut args) => args.entrypoint(),
    CandidateCommand::PM        (mut args) => args.entrypoint(),
    CandidateCommand::REPL      (mut args) => args.entrypoint(),
    CandidateCommand::Test      (mut args) => args.entrypoint(),
  }
  .unwrap_or_else(exit_with_error);
}

/// Prints the error message simply and exits the process once error occurred.
/// The application must do the error handling itself.
fn exit_with_error(error: VspError) {
  eprintln!("{}", error);
  std::process::exit(exitcode::EXIT_FAILURE);
}

/// Get a designed ANSI style.
fn get_styles() -> Styles {
  Styles::styled().literal(
    anstyle::Style::new()
      .bold()
      .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
  )
}
