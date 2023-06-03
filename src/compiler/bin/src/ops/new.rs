use core::str::FromStr;
use std::path::PathBuf;

use clap::arg;
use clap::builder::PossibleValuesParser;
use clap::Args;
use vsp_error::VspResult;
use vsp_pm::new::NewProjectConfig;
use vsp_pm::vcs::VersionControl;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Project name
  #[arg()]
  project: String,
  /// Path to the project
  #[arg(long)]
  path:    Option<PathBuf>,
  /// Version control service. Initialize the project with given version control system.
  #[arg(long, value_parser = get_vcs_value_parser())]
  vcs:     Option<String>,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    let vcs = self.vcs.clone().map(|s| VersionControl::from_str(s.as_str()).unwrap());
    let config = NewProjectConfig::new(&self.project, vcs, self.path.clone());
    config.create_new_project()
  }
}

#[cfg(not(target_env = "musl"))]
fn get_vcs_value_parser() -> PossibleValuesParser {
  PossibleValuesParser::new(["git", "fossil", "hg", "svn"])
}

#[cfg(target_env = "musl")]
fn get_vcs_value_parser() -> PossibleValuesParser {
  PossibleValuesParser::new(["git"])
}
