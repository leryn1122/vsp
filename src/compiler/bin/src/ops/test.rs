use crate::CommandLine;

pub(crate) fn cli() -> clap::Command {
  clap::Command::new("test").about("Run all unit and integration tests")
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut CommandLine, args: &clap::ArgMatches) -> anyhow::Result<()> {
  todo!()
}
