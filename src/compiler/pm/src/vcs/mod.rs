pub(crate) mod fossil;
pub(crate) mod git;
pub(crate) mod hg;
pub(crate) mod svn;

use std::str::FromStr;

use anyhow::anyhow;

/// Version control toolchains.
pub enum VersionControl {
  Git,
  Fossil,
  Hg,
  Svn,
}

impl FromStr for VersionControl {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "git" => Ok(VersionControl::Git),
      "fossil" => Ok(VersionControl::Fossil),
      "hg" => Ok(VersionControl::Hg),
      "svn" => Ok(VersionControl::Svn),
      s => Err(anyhow!("Unsupported version control: {}", s)),
    }
  }
}
