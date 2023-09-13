use semver::Version;

use crate::dep::Dependencies;

#[allow(dead_code)]
pub struct Manifest {
  project:      Project,
  dependencies: Dependencies,
}

impl Manifest {
  fn new(name: impl Into<String>) -> Self {
    Self {
      project:      Project {
        name:    name.into(),
        version: Version::parse("0.1.0").unwrap(),
      },
      dependencies: vec![],
    }
  }
}

impl Default for Manifest {
  //noinspection SpellCheckingInspection
  fn default() -> Self {
    Self::new("helloworld")
  }
}

// #[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub(crate) struct Project {
  name:    String,
  version: Version,
}

#[cfg(test)]
mod tests {
  /// Tests write the project config file to the temporary directory.
  #[test]
  fn test_manifest() {
    let temp_dir = std::env::temp_dir();
  }
}
