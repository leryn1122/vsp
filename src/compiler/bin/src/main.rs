use clap::Parser;
use clap::Subcommand;
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
#[command(after_help = format!(resources_str!("help.txt"), url = vsp_bin::REPORT_URL))]
pub struct MainCommand {
  #[command(subcommand)]
  subcommand: CandidateCommand,
}

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
  let command = MainCommand::parse();
  match command.subcommand {
    CandidateCommand::Clean(args) => args.entrypoint(),
    CandidateCommand::Compile(args) => args.entrypoint(),
    CandidateCommand::Completion(args) => args.entrypoint(),
    CandidateCommand::Debug(args) => args.entrypoint(),
    CandidateCommand::Dump(args) => args.entrypoint(),
    CandidateCommand::LSP(args) => args.entrypoint(),
    CandidateCommand::New(args) => args.entrypoint(),
    CandidateCommand::PM(args) => args.entrypoint(),
    CandidateCommand::REPL(args) => args.entrypoint(),
    CandidateCommand::Test(args) => args.entrypoint(),
  }
  .unwrap_or_else(exit_with_error);
}

fn exit_with_error(error: anyhow::Error) {
  eprintln!("{}", error);
  std::process::exit(exitcode::EXIT_FAILURE);
}
