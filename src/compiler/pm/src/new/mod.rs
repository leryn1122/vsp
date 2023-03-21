use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use anyhow::anyhow;

use crate::vcs::fossil::FossilRepo;
use crate::vcs::git::GitRepo;
use crate::vcs::hg::HgRepo;
use crate::vcs::svn::SvnRepo;
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
      vcs:  vcs,
      path: path,
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

    match std::fs::create_dir(path.clone()) {
      Ok(_) => {}
      Err(e) => {
        // Fast return if failed.
        return Err(anyhow!("Fail to create project: {}", self.name));
      }
    }

    // `cd` into project directory.
    std::env::set_current_dir(&path).unwrap();

    for entry in vec!["build.vsp", "lib.vsp", "module.vsp", "manifest.toml"] {
      let res = OpenOptions::new().create_new(true).write(true).open(entry);
      match res {
        Ok(mut f) => {
          f.write_all(match entry {
            "build.vsp" => include_bytes!("build.vsp"),
            "lib.vsp" => include_bytes!("lib.vsp"),
            "module.vsp" => include_bytes!("module.vsp"),
            "manifest.toml" => include_bytes!("manifest.toml"),
            _ => unreachable!(),
          })
          .map_err(|e| anyhow!(e))
          .expect("Failed to create file.");
        }
        Err(e) => return Err(anyhow!(e)),
      }
    }

    let cwd = std::env::current_dir().unwrap();
    let cwd = cwd.as_path();
    let project = PathBuf::from(&self.name);
    let project = project.as_path();
    match self.vcs {
      None => { /* Ignore */ }
      Some(VersionControl::Git) => GitRepo::init(project, cwd).unwrap(),
      Some(VersionControl::Fossil) => FossilRepo::init(project, cwd).unwrap(),
      Some(VersionControl::Hg) => HgRepo::init(project, cwd).unwrap(),
      Some(VersionControl::Svn) => SvnRepo::init(project, cwd).unwrap(),
    }

    Ok(())
  }
}
