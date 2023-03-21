use std::path::Path;

use anyhow::Result;

pub(crate) struct GitRepo;

impl GitRepo {
  /// Using `git2` crate to initialize the repository.
  pub(crate) fn init(path: &Path, _: &Path) -> Result<()> {
    git2::Repository::init(path).unwrap();
    Ok(())
  }
}
