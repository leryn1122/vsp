use semver::Version;

#[allow(dead_code)]
// #[derive(Deserialize, Serialize, Debug)]
pub struct Manifest {
  project: Project,
  // dependencies: Dependencies,
}

impl Manifest {
  fn new(name: impl Into<String>) -> Self {
    let project = Project {
      name:    name.into(),
      version: Version::parse("0.1.0").unwrap(),
    };

    Self { project: project }
  }
}

impl Default for Manifest {
  //noinspection SpellCheckingInspection
  fn default() -> Self {
    Self::new("helloworld")
  }
}

// #[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Project {
  name:    String,
  version: Version,
}

#[cfg(test)]
mod tests {

  /// Tests write the project config file to the temporary directory.
  #[test]
  pub fn test_manifest() {
    let temp_dir = std::env::temp_dir();
  }
}
