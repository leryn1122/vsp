use std::str::FromStr;

use anyhow::anyhow;

pub enum VersionControl {
  Git,
  SVN,
  Fossil,
}

impl FromStr for VersionControl {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "git" => Ok(VersionControl::Git),
      "svn" => Ok(VersionControl::SVN),
      "fossil" => Ok(VersionControl::Fossil),
      s => Err(anyhow!("Unsupported version control: {}", s)),
    }
  }
}
