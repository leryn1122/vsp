use clap::arg;
use clap::ArgMatches;
use clap::Command;

pub(crate) fn cli(alias: bool) -> Command {
  let command = if alias {
    Command::new("vsplsp")
  } else {
    Command::new("lsp")
  };
  command
    .about("Language server")
    .arg_required_else_help(true)
    .args(&[arg!(--"print-config-schema" "Print configuration schema.")])
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  Ok(())
}
