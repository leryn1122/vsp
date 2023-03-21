use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use clap::arg;
use clap::value_parser;
use clap::Command;
use vsp_pm::new::NewProjectConfig;
use vsp_pm::vcs::VersionControl;

use crate::Config;

pub(crate) fn cli() -> Command {
  Command::new("new")
    .about("Create new project")
    .arg_required_else_help(true)
    .arg(
      arg!([project] "Project name")
        .required(true)
        .value_parser(value_parser!(String)),
    )
    .arg(
      arg!(--vcs [vcs] "Version control service. Initialize the project with given version control \
system. Possible value: `git', `fossil', `hg', `svn', none."),
    )
    .arg(arg!(--path [path] "Path to the project.").value_parser(value_parser!(PathBuf)))
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &clap::ArgMatches) -> anyhow::Result<()> {
  let project = args.get_one::<String>("project").unwrap();
  let vcs = args
    .get_one::<String>("vcs")
    .ok_or(anyhow!("Unsupported version control service"))
    .and_then(|s| VersionControl::from_str(s))
    .ok();
  let path = args
    .get_one::<PathBuf>("path")
    .unwrap_or(&std::env::current_dir().unwrap())
    .to_path_buf();
  let config = NewProjectConfig::new(project, vcs, path);
  config.create_new_project()
}
