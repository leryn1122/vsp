use cli::CommandLine;

pub mod cli;
pub mod ops;

/// All the command line entries point.
fn main() {
  let mut commandline = CommandLine::default();
  cli::run(&mut commandline).unwrap_or_else(|e| cli::exit_with_error(e));
}
