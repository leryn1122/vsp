use std::path::Path;

use anyhow::Result;
use vsp_support::process::ProcessBuilder;

pub(crate) struct HgRepo;

impl HgRepo {
  /// Delegate the command `hg init -- <project>` to the process invocation.
  pub(crate) fn init(path: &Path, cwd: &Path) -> Result<()> {
    ProcessBuilder::new("hg")
      .cwd(cwd)
      .arg("init")
      .arg("--")
      .arg(path)
      .exec()?;
    Ok(())
  }
}
