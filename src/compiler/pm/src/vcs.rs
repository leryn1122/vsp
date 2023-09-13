use std::path::Path;
use std::str::FromStr;

use vsp_error::VspError;
use vsp_error::VspResult;
use vsp_support::process::ProcessBuilder;

/// Version control toolchains.
#[derive(Clone)]
pub enum VersionControl {
  Git,
  #[cfg(not(target_env = "musl"))]
  Fossil,
  #[cfg(not(target_env = "musl"))]
  Hg,
  #[cfg(not(target_env = "musl"))]
  Svn,
}

impl FromStr for VersionControl {
  type Err = VspError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    #[cfg(not(target_env = "musl"))]
    match s {
      "git" => Ok(VersionControl::Git),
      "fossil" => Ok(VersionControl::Fossil),
      "hg" => Ok(VersionControl::Hg),
      "svn" => Ok(VersionControl::Svn),
      s => Err(VspError::new(format!("Unsupported version control: {}", s))),
    }

    #[cfg(target_env = "musl")]
    match s {
      "git" => Ok(VersionControl::Git),
      s => Err(VspError::new("Unsupported version control: {}", s)),
    }
  }
}

/// Empty struct for `git`.
pub(crate) struct GitRepo;

impl GitRepo {
  /// Using `git2` crate to initialize the repository, or to delegate to `git` command on `musl`
  /// platform.
  #[allow(unused_variables)]
  pub(crate) fn init(path: &Path, cwd: &Path) -> VspResult<()> {
    #[cfg(not(target_env = "musl"))]
    let res = git2::Repository::init(path).map(|_| ()).map_err(VspError::from);
    #[cfg(target_env = "musl")]
    let res = vsp_support::process::ProcessBuilder::new("git")
      .cwd(cwd)
      .arg("init")
      .arg(path)
      .exec()
      .map_err(VspError::from);
    res
  }
}

/// Empty struct for `fossil`.
#[cfg(not(target_env = "musl"))]
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
  pub fn init(path: &Path, cwd: &Path) -> VspResult<()> {
    ProcessBuilder::new("fossil")
      .cwd(cwd)
      .arg("init")
      .arg("--")
      .arg(path)
      .exec()
      .map_err(VspError::from)
  }
}

/// Empty struct for `hg`.
#[cfg(not(target_env = "musl"))]
pub(crate) struct HgRepo;

impl HgRepo {
  /// Delegate the command `hg init -- <project>` to the process invocation.
  pub(crate) fn init(path: &Path, cwd: &Path) -> VspResult<()> {
    ProcessBuilder::new("hg")
      .cwd(cwd)
      .arg("init")
      .arg("--")
      .arg(path)
      .exec()
      .map_err(VspError::from)
  }
}

/// Empty struct for `svn`.
#[cfg(not(target_env = "musl"))]
pub(crate) struct SvnRepo;

impl SvnRepo {
  pub(crate) fn init(_path: &Path, _cwd: &Path) -> VspResult<()> {
    ProcessBuilder::new("svn").arg("").exec().map_err(VspError::from)
  }
}
