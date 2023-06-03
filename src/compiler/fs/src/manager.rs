use crate::vfs::get_real_file_system;
use crate::vfs::path::VFSPath;
use crate::vfs::FileObject;
use crate::vfs::FileSystem;
use crate::vfs::VFSWrapper;

pub struct VFSManager {
  vfs: VFSWrapper,
}

impl VFSManager {
  pub fn get_file(&self, file: &VFSPath) -> Option<Box<dyn FileObject>> {
    None
  }
}

impl VFSManager {
  pub fn from<VFS>(vfs: VFS) -> VFSManager
  where
    VFS: FileSystem + Sized,
  {
    Self {
      vfs: VFSWrapper::from(vfs),
    }
  }
}

impl Default for VFSManager {
  fn default() -> Self {
    let fs = get_real_file_system();
    Self {
      vfs: VFSWrapper::from(fs),
    }
  }
}
