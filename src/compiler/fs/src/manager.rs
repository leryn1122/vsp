use crate::path::VFSPath;
use crate::vfs;
use crate::vfs::FileObject;
use crate::vfs::FileSystem;
use crate::vfs::VFSWrapper;

/// Manager for VFS, virtual filesystem.
pub struct VFSManager {
  vfs: VFSWrapper,
}

impl VFSManager {
  pub fn get_file(&self, _file: &VFSPath) -> Option<Box<dyn FileObject>> {
    let vfs = self.as_ref();
    None
  }
}

impl AsRef<dyn FileSystem> for VFSManager {
  fn as_ref(&self) -> &dyn FileSystem {
    self.vfs.vfs.as_ref()
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
    let fs = vfs::get_real_file_system();
    Self {
      vfs: VFSWrapper::from(fs),
    }
  }
}
