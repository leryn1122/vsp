pub trait FileManager {}

pub struct DefaultFileManager<VFS> {
  vfs: VFS,
}
