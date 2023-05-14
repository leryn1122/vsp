use clap::arg;
use clap::ArgMatches;
use clap::Command;
use vsp_support::resources_str;

pub(crate) fn cli(alias: bool) -> Command {
  let command = if alias {
    Command::new("vsplsp")
  } else {
    Command::new("lsp")
  };
  command
    .about("Language server based on LSP (language server protocol)")
    .arg_required_else_help(true)
    .args(&[
      arg!(-d --daemon "Run as a daemon"),
      arg!(--"print-config-schema" "Print configuration schema"),
    ])
    .after_help(resources_str!("lsp/help.txt"))
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  let daemon = args.get_flag("daemon");
  let print_config_schema = args.get_flag("print-config-schema");
  Ok(())
}
