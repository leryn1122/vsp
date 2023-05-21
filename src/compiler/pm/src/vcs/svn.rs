use std::path::Path;

use vsp_error::VspResult;

pub(crate) struct SvnRepo;

impl SvnRepo {
  pub(crate) fn init(_path: &Path, _cwd: &Path) -> VspResult<()> {
    todo!();
    // ProcessBuilder::new("svn").arg("").exec()?;
    // Ok(SvnRepo)
  }
}
