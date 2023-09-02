use std::path::Path;
use std::path::PathBuf;

/// File seperator.
pub(crate) const SEPERATOR: char = '/';
pub(crate) const SEPERATOR_STR: &str = "/";

/// Path implementation used for the virtual file system.
#[derive(Clone, Debug)]
pub struct VFSPath {
  inner: String,
}

impl VFSPath {
  pub fn as_str(&self) -> &str {
    &self.inner
  }

  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }
}

impl VFSPath {
  pub fn from_str(s: &str) -> Self {
    Self {
      inner: s.to_string(),
    }
  }

  /// Construct a new VFS path from `OsString`.
  pub fn from(path: impl Into<String>) -> Self {
    Self { inner: path.into() }
  }

  /// Returns root path.
  pub fn root() -> Self {
    Self::from(SEPERATOR)
  }

  /// Returns true if this is the root path.
  pub fn is_root(&self) -> bool {
    self.inner.is_empty() || self.inner == SEPERATOR_STR
  }

  pub fn starts_with(&self, path: impl Into<String>) -> bool {
    todo!()
  }

  pub fn to_path_buf(&self) -> PathBuf {
    todo!()
  }

  pub fn is_absolute(&self) -> bool {
    self.inner.starts_with(SEPERATOR)
  }

  ///
  pub fn join(&self, path: impl AsRef<str>) -> Self {
    self.join_internal(path.as_ref())
  }

  fn join_internal(&self, path: &str) -> Self {
    if path.is_empty() {
      return self.to_owned();
    }
    let mut new_segments: Vec<&str> = vec![];
    let mut base_path = if path.starts_with(SEPERATOR) {
      Self::root()
    } else {
      self.to_owned()
    };

    if path.len() > 1 && path.ends_with(SEPERATOR) {
      todo!()
    }
    for segment in path.split(SEPERATOR) {
      if segment == "." || segment.is_empty() {
        continue;
      }
      if segment == ".." {
        if !new_segments.is_empty() {
          new_segments.truncate(new_segments.len() - 1);
        } else {
          base_path = base_path.parent();
        }
      } else {
        new_segments.push(segment);
      }
    }

    let mut path = base_path.inner;
    for segments in new_segments {
      path += SEPERATOR_STR;
      path += segments;
    }

    return Self::from(path);
  }

  /// Returns the parent path.
  pub fn parent(&self) -> Self {
    let index = self.inner.rfind(SEPERATOR);
    index
      .map(|idx| Self {
        inner: self.inner[..idx].to_string(),
      })
      .unwrap_or_else(|| Self::root())
  }
}

impl AsRef<Path> for VFSPath {
  fn as_ref(&self) -> &Path {
    self.inner.as_ref()
  }
}
