use crate::Config;

pub(crate) fn cli() -> clap::Command {
  clap::Command::new("run")
    .about("Language runtime launcher")
    .arg_required_else_help(true)
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &clap::ArgMatches) -> anyhow::Result<()> {
  todo!()
}
