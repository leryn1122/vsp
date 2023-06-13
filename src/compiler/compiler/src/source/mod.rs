use vsp_fs::vfs::FileObject;

pub struct SourceManager {}

impl SourceManager {
  pub fn create_main_file_id(&mut self, file: &mBox<dyn FileObject>) {}
}

impl Default for SourceManager {
  fn default() -> Self {
    Self {}
  }
}
