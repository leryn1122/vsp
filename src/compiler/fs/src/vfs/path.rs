use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) const SEPERATOR: char = '/';

///
pub struct VFSPath {
  inner: OsString,
}

impl VFSPath {
  /// Construct a new VFS path from `OsString`.
  pub fn from(path: impl Into<OsString>) -> Self {
    Self { inner: path.into() }
  }

  pub fn starts_with(&self, path: impl Into<OsString>) -> bool {
    todo!()
  }

  pub fn to_path_buf(&self) -> PathBuf {
    todo!()
  }

  pub fn is_absolute(&self) -> bool {
    self.inner.to_string_lossy().starts_with(SEPERATOR)
  }
}
