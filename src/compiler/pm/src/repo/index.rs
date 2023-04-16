use vsp_support::semver::Version;

pub struct IndexFile {
  api_version: Version,
  entries:     Vec<Entry>,
  public_key:  String,
}

pub struct Entry {
  name: String,
}
