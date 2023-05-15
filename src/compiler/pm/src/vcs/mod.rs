pub(crate) mod git;

#[cfg(not(target_env = "musl"))]
pub(crate) mod fossil;
#[cfg(not(target_env = "musl"))]
pub(crate) mod hg;
#[cfg(not(target_env = "musl"))]
pub(crate) mod svn;

use std::str::FromStr;

use anyhow::anyhow;

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
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    #[cfg(not(target_env = "musl"))]
    match s {
      "git" => Ok(VersionControl::Git),
      "fossil" => Ok(VersionControl::Fossil),
      "hg" => Ok(VersionControl::Hg),
      "svn" => Ok(VersionControl::Svn),
      s => Err(anyhow!("Unsupported version control: {}", s)),
    }

    #[cfg(target_env = "musl")]
    match s {
      "git" => Ok(VersionControl::Git),
      s => Err(anyhow!("Unsupported version control: {}", s)),
    }
  }
}
