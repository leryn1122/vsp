use semver::Version;

#[allow(dead_code)]
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

pub(crate) struct Project {
  name:    String,
  version: Version,
}
