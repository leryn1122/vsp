pub(crate) fn cli() -> clap::Command {
  clap::Command::new("test").about("Run all unit and integration tests")
}

#[allow(unused_variables)]
pub(crate) fn execute(args: &clap::ArgMatches) -> anyhow::Result<()> {
  todo!()
}
