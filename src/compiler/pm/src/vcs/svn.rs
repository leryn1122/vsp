use std::path::Path;

use anyhow::Result;

pub(crate) struct SvnRepo;

impl SvnRepo {
  pub(crate) fn init(_path: &Path, _cwd: &Path) -> Result<()> {
    todo!();
    // ProcessBuilder::new("svn").arg("").exec()?;
    // Ok(SvnRepo)
  }
}
