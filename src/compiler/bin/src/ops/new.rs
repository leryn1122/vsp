use core::str::FromStr;
use std::path::PathBuf;

use anyhow::anyhow;
use clap::arg;
use clap::builder::PossibleValuesParser;
use clap::value_parser;
use clap::ArgMatches;
use clap::Command;
use clap::ValueHint;
use vsp_pm::new::NewProjectConfig;
use vsp_pm::vcs::VersionControl;

pub(crate) fn cli(_: bool) -> Command {
  Command::new("new")
    .about("Create new project")
    .arg_required_else_help(true)
    .args(&[
    arg!([project] "Project name")
      .required(true)
      .value_parser(value_parser!(String)),
    arg!(--vcs <vcs> "Version control service. Initialize the project with given version control \
system.")
    .value_parser(get_vcs_value_parser()),
    arg!(--path <path> "Path to the project.")
      .value_parser(value_parser!(PathBuf))
      .value_hint(ValueHint::AnyPath),
  ])
}

pub(crate) fn entrypoint(args: &ArgMatches) -> anyhow::Result<()> {
  let project = args.get_one::<String>("project").unwrap();
  // No need to use `PossibleValuesParser` here.
  // Use `FromStr` tricky.
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

#[cfg(not(target_env = "musl"))]
fn get_vcs_value_parser() -> PossibleValuesParser {
  PossibleValuesParser::new(vec!["git", "fossil", "hg", "svn"])
}

#[cfg(target_env = "musl")]
fn get_vcs_value_parser() -> PossibleValuesParser {
  PossibleValuesParser::new(vec!["git"])
}
