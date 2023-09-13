pub type Dependencies = Vec<Dependency>;

pub struct Dependency {
  name:    String,
  version: semver::Version,
}
