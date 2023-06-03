pub mod ops;
pub mod support;

use clap::builder::Styles;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
#[command(propagate_version = true)]
#[command(styles = get_styles())]
pub struct MainCommand {
  #[command(subcommand)]
  subcommand: BuildCommand,
}

#[derive(Subcommand)]
pub enum BuildCommand {
  ///
  Build,
}

fn main() {
  let command = MainCommand::parse();

  let res = match command.subcommand {
    BuildCommand::Build => ops::build(),
  };
  res
    .map_err(|e| {
      eprintln!("{}", e.to_string());
      std::process::exit(1);
    })
    .unwrap()
}

/// Get a designed ANSI style.
fn get_styles() -> Styles {
  Styles::styled().literal(
    anstyle::Style::new()
      .bold()
      .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
  )
}
