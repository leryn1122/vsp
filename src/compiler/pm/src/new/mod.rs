use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use anyhow::anyhow;
use vsp_support::resources_bytes;

#[rustfmt::skip]
#[cfg(not(target_env = "musl"))]
use crate::vcs::{
  fossil::FossilRepo,
  hg::HgRepo,
  svn::SvnRepo
};
use crate::vcs::git::GitRepo;
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
  pub fn new(name: impl Into<String>, vcs: Option<VersionControl>, path: PathBuf) -> Self {
    Self {
      name: name.into(),
      vcs,
      path,
    }
  }

  /// Create new project by given configuration.
  /// It is about to do 3 actions in this function.
  /// - Create the empty dir named after project.
  /// - Create the list of files.
  /// - Write the content of files.
  pub fn create_new_project(&self) -> anyhow::Result<()> {
    let path = self.path.join(&self.name);
    if path.exists() {
      return Err(anyhow!("Directory has already existed."));
    }

    // Use a hack to create the project directory by vcs.
    let cwd = std::env::current_dir().unwrap();
    let cwd = cwd.as_path();
    let project = PathBuf::from(&self.name);
    let project = project.as_path();

    match self.vcs {
      None => std::fs::create_dir(path.clone()).expect("Fail to create project: {}"),
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
          f.write_all(match *entry {
            "build.vsp" => resources_bytes!("new/build.vsp"),
            "lib.vsp" => resources_bytes!("new/lib.vsp"),
            "module.vsp" => resources_bytes!("new/module.vsp"),
            "manifest.toml" => resources_bytes!("new/manifest.toml"),
            _ => unreachable!(),
          })
          .map_err(|e| anyhow!(e))
          .expect("Failed to create file.");
        }
        Err(e) => return Err(anyhow!(e)),
      }
    }

    Ok(())
  }
}
