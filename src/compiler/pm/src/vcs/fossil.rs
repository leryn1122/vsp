use std::path::Path;

use vsp_error::VspResult;

pub(crate) struct FossilRepo;

impl FossilRepo {
  ///
  ///
  /// ```bash
  /// fossil init <project>.fossil
  /// # Return `project-id`, `server-id` and `admin-user` here.
  ///
  /// fossil open ../example.fossil
  /// ```
  pub fn init(_path: &Path, _cwd: &Path) -> VspResult<()> {
    todo!();

    // ProcessBuilder::new("fossil")
    //   .cwd(cwd)
    //   .arg("init")
    //   .arg("--")
    //   .arg(path)
    //   .exec()?;
    // Ok(FossilRepo)
  }
}
