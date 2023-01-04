use std::fs;

use anyhow::anyhow;

use crate::Config;

pub(crate) fn cli() -> clap::Command {
  clap::Command::new("clean").about("Clean target directory")
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &clap::ArgMatches) -> anyhow::Result<()> {
  let target_dir = config.cwd.join("target");
  let res = fs::remove_dir_all(target_dir);
  match res {
    Ok(_) => Ok(()),
    Err(e) => Err(anyhow!(e)),
  }
}
