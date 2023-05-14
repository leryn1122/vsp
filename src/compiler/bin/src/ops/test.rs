use clap::arg;

pub(crate) fn cli(_: bool) -> clap::Command {
  clap::Command::new("test")
    .about("Run all unit tests and integration tests")
    .args(&[arg!(--unittests "Run all unit tests only")])
}

#[allow(unused_variables)]
pub(crate) fn entrypoint(args: &clap::ArgMatches) -> anyhow::Result<()> {
  let unittests = args.get_flag("unittests");
  Ok(())
}
