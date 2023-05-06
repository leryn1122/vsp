use clap::ArgMatches;
use clap::Command;

pub(crate) fn cli(alias: bool) -> Command {
  let command = if alias {
    Command::new("vspdb")
  } else {
    Command::new("debug")
  };
  command.about("Native debugger").arg_required_else_help(true)
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  todo!()
}
