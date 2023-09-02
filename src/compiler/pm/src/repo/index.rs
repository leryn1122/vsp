use semver::Version;

#[allow(dead_code)]
pub struct IndexFile {
  api_version: Version,
  entries:     Vec<Entry>,
  public_key:  String,
}

#[allow(dead_code)]
pub struct Entry {
  name: String,
}
