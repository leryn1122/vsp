use std::path::Path;

use anyhow::Result;

pub(crate) struct GitRepo;

impl GitRepo {
  /// Using `git2` crate to initialize the repository, or to delegate to `git` command on `musl`
  /// platform.
  #[allow(unused_variables)]
  pub(crate) fn init(path: &Path, cwd: &Path) -> Result<()> {
    #[cfg(not(target_env = "musl"))]
    git2::Repository::init(path).unwrap();

    #[cfg(target_env = "musl")]
    vsp_support::process::ProcessBuilder::new("git")
      .cwd(cwd)
      .arg("init")
      .arg(path)
      .exec()?;

    Ok(())
  }
}
