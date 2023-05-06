use clap::ArgMatches;
use clap::Command;

pub(crate) fn cli(alias: bool) -> Command {
  let command = if alias {
    Command::new("vspm")
  } else {
    Command::new("pm")
  };
  command.about("Project manager").arg_required_else_help(true)
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  todo!()
}
