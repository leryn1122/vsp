use std::str::FromStr;

use clap::{arg, value_parser};

use vsp_packager::project::ProjectConfig;
use vsp_packager::vcs::VersionControl;

use crate::Config;

pub(crate) fn cli() -> clap::Command {
  clap::Command::new("new")
    .about("Create new project")
    .arg_required_else_help(true)
    .arg(
      arg!([project] "Project name")
        .required(true)
        .value_parser(value_parser!(String)),
    )
    .arg(
      arg!(--vcs [vsc] "Version control. Initialize the project with given version control \
system. Possible value: git, fossil, none."),
    )
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &clap::ArgMatches) -> anyhow::Result<()> {
  let name = args.get_one::<String>("project").unwrap();
  let mut project_config = ProjectConfig::new(name);
  project_config.vcs = if let Some(vcs) = args.get_one::<String>("vcs") {
    match VersionControl::from_str(vcs) {
      Ok(vcs) => Some(vcs),
      Err(_) => None,
    }
  } else {
    None
  };
  project_config.create_new_project()
}
