use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::anyhow;

use crate::vcs::VersionControl;

/// Project configuration.
pub struct ProjectConfig {
  pub name: String,
  pub vcs: Option<VersionControl>,
  pub path: PathBuf,
}

impl ProjectConfig {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      vcs: None,
      path: PathBuf::new(),
    }
  }

  /// Create new project by given configuration.
  pub fn create_new_project(&self) -> anyhow::Result<()> {
    let project = PathBuf::from(self.name.clone());
    if let Err(e) = std::fs::create_dir_all(project.clone()) {
      eprintln!("Fail to create project as: {}", self.name);
      return Err(anyhow!(e));
    }

    // `cd` into project
    std::env::set_current_dir(project).unwrap();

    let entries = vec!["build.vsp", "lib.vsp", "module.vsp", "manifest.yaml"];
    for entry in entries {
      let res = File::create(entry);
      return match res {
        Ok(mut f) => f
          .write_all(include_bytes!("build.vsp"))
          .map_err(|e| anyhow!(e)),
        Err(e) => {
          eprintln!("Fail to create file: {}", entry);
          Err(anyhow!(e))
        }
      };
    }
    Ok(())
  }

  fn write_manifest() -> anyhow::Result<()> {
    Ok(())
  }
}
