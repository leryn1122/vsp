use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use vsp_error::VspError;
use vsp_error::VspResult;
use vsp_support::resources_bytes;

#[rustfmt::skip]
#[cfg(not(target_env = "musl"))]
use crate::vcs::{
  FossilRepo,
  HgRepo,
  SvnRepo,
};
use crate::vcs::GitRepo;
use crate::vcs::VersionControl;

/// Configuration for creating a brand-new project.
///
/// ```bash
/// vsp new helloworld --vcs git --path /path/to/project
/// ```
pub struct NewProjectConfig {
  name: String,
  vcs:  Option<VersionControl>,
  path: PathBuf,
}

impl NewProjectConfig {
  pub fn new(name: impl Into<String>, vcs: Option<VersionControl>, path: Option<PathBuf>) -> Self {
    Self {
      name: name.into(),
      vcs,
      path: path.unwrap_or(std::env::current_dir().unwrap()),
    }
  }

  /// Create new project by given configuration.
  /// It is about to do 3 actions in this function.
  /// - Create the empty dir named after project.
  /// - Create the list of files.
  /// - Write the content of files.
  pub fn create_new_project(&self) -> VspResult<()> {
    let path = self.path.join(&self.name);
    if path.exists() {
      return Err(VspError::new(
        "Failed to create project: the directory has already existed.",
      ));
    }

    // Use a hack to create the project directory by vcs.
    let cwd = std::env::current_dir().unwrap();
    let cwd = cwd.as_path();
    let project = PathBuf::from(&self.name);
    let project = project.as_path();
    match self.vcs {
      None => std::fs::create_dir(&path)
        .unwrap_or_else(|e| panic!("Fail to create project [{}]: {}", &self.name, e)),
      Some(VersionControl::Git) => GitRepo::init(project, cwd).unwrap(),
      #[cfg(not(target_env = "musl"))]
      Some(VersionControl::Fossil) => FossilRepo::init(project, cwd).unwrap(),
      #[cfg(not(target_env = "musl"))]
      Some(VersionControl::Hg) => HgRepo::init(project, cwd).unwrap(),
      #[cfg(not(target_env = "musl"))]
      Some(VersionControl::Svn) => SvnRepo::init(project, cwd).unwrap(),
    }

    // `cd` into project directory.
    std::env::set_current_dir(&path).unwrap();
    for entry in &["build.vsp", "lib.vsp", "module.vsp", "manifest.toml"] {
      let res = OpenOptions::new().create_new(true).write(true).open(entry);
      match res {
        Ok(mut f) => {
          #[rustfmt::skip]
          f.write_all(match *entry {
            "build.vsp"     => resources_bytes!("new/build.vsp"),
            "lib.vsp"       => resources_bytes!("new/lib.vsp"),
            "module.vsp"    => resources_bytes!("new/module.vsp"),
            "manifest.toml" => resources_bytes!("new/manifest.toml"),
            _ => unreachable!(),
          })
          .map_err(VspError::from)
          .unwrap_or_else(|e| panic!("Failed to create file [{}]: {}", entry, e));
        }
        Err(e) => return Err(VspError::from(e)),
      }
    }

    Ok(())
  }
}
