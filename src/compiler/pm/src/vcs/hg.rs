use std::path::Path;

use vsp_error::VspResult;
use vsp_support::process::ProcessBuilder;

pub(crate) struct HgRepo;

impl HgRepo {
  /// Delegate the command `hg init -- <project>` to the process invocation.
  pub(crate) fn init(path: &Path, cwd: &Path) -> VspResult<()> {
    ProcessBuilder::new("hg").cwd(cwd).arg("init").arg("--").arg(path).exec()
  }
}
