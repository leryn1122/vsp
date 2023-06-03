use std::sync::Arc;

/// # Compilation Database
///
/// A database to record the source files for incremental compilation.
pub struct CompilationDatabase;

impl CompilationDatabase {
  pub fn load_from_directory(base_dir: String, err_msg: impl Into<String>) -> Option<Arc<Self>> {
    let instance = Self {};
    Some(Arc::new(instance))
  }

  pub fn auto_detect_from_source(source: String, err_msg: impl Into<String>) -> Option<Arc<Self>> {
    todo!()
  }

  pub fn auto_detect_from_directory(
    base_dir: String,
    err_msg: impl Into<String>,
  ) -> Option<Arc<Self>> {
    todo!()
  }

  pub fn get_all_files() -> Vec<String> {
    todo!()
  }
}
