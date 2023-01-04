use std::path::PathBuf;

use crate::fs::manager::{FileObject, Kind};

/// Source file manager.
#[derive(Debug)]
pub struct NormalSourceFileManager {
  root_dir: PathBuf,
}

impl NormalSourceFileManager {
  pub fn new() -> Self {
    Self {
      root_dir: PathBuf::new(),
    }
  }
}

pub struct NormalFileObject {}

impl FileObject for NormalFileObject {
  fn get_kind() -> Kind {
    Kind::Source
  }
}
