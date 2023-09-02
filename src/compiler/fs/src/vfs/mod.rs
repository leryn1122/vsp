use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use getset::Getters;
use getset::MutGetters;
use vsp_error::VspError;
use vsp_error::VspResult;
use vsp_support::exitcode;
use vsp_support::exitcode::ExitCode;

use crate::path::VFSPath;

/// Abstract file object trait for the virtual filesystem.
pub trait FileObject {
  fn get_name(&self) -> Result<OsString, ()>;

  fn close(&mut self) -> ExitCode;

  fn read_to_string(&mut self, buf: &mut String) -> VspResult<()>;
}

/// File type for file objects on filesystem.
#[derive(Debug, PartialEq)]
pub enum VFSFileType {
  RegularFile,
  DirectoryFile,
  #[cfg(unix)]
  SymbolicLink,
  /** A unix socket on local machine */
  #[cfg(unix)]
  SocketFile,
  /** Result for finding a file. */
  FileNotFound,
  /** Result for unknown or unexpected case. */
  Unknown,
}

enum AccessMode {
  Read,
  Write,
  Execute,
}

/// File metadata type for file objects on virtual filesystem.
#[derive(Debug)]
pub struct VFSMetadata {
  file_type: VFSFileType,
  len:       u64,
}

impl VFSMetadata {
  pub fn exists(&self) -> bool {
    ![VFSFileType::FileNotFound, VFSFileType::Unknown].contains(&self.file_type)
  }

  pub fn is_directory(&self) -> bool {
    self.file_type == VFSFileType::DirectoryFile
  }

  pub fn is_regular_file(&self) -> bool {
    self.file_type == VFSFileType::RegularFile
  }

  pub fn is_symlink(&self) -> bool {
    self.file_type == VFSFileType::SymbolicLink
  }

  pub fn is_other(&self) -> bool {
    ![VFSFileType::RegularFile, VFSFileType::DirectoryFile].contains(&self.file_type)
  }

  pub fn is_unknown(&self) -> bool {
    self.file_type == VFSFileType::Unknown
  }
}

/// A member of a directory, used for `DirectoryIterator`, carries the necessary information for all
/// the file system implementations.
#[derive(Getters, MutGetters)]
pub struct DirectoryEntry {
  #[getset(get)]
  path:      VFSPath,
  #[getset(get)]
  file_type: VFSFileType,
  // #[getset(get)]
  // follow_symlinks: bool,
}

impl DirectoryEntry {
  fn new(path: VFSPath, file_type: VFSFileType) -> Self {
    Self { path, file_type }
  }
}

/// An iterator over a directory.
pub trait DirectoryEntryIterator: Iterator<Item = DirectoryEntry> {}

pub(crate) struct DirectoryEntryIteratorImpl {
  current: DirectoryEntry,
}

impl DirectoryEntryIteratorImpl {}

impl Iterator for DirectoryEntryIteratorImpl {
  type Item = DirectoryEntry;

  fn next(&mut self) -> Option<Self::Item> {
    todo!()
  }
}

impl DirectoryEntryIterator for DirectoryEntryIteratorImpl {}

/// **DON'T USE IT**
///
/// An internal fat pointer of the virtual file system.
///
/// @see `crate::vfs::FileSystem`
#[doc(hidden)]
pub(crate) struct VFSWrapper {
  pub(crate) vfs: Box<dyn FileSystem>,
}

impl VFSWrapper {
  pub(crate) fn from<VFS>(vfs: VFS) -> Self
  where
    VFS: FileSystem + Sized,
  {
    Self { vfs: Box::new(vfs) }
  }
}

/// # Virtual File System
///
/// A virtual file system is a memory object that represents an abstract interface to provide IO
/// streams with the consistent API over the real filesystem, in-memory filesystem, embedded
/// filesystem, etc.
///
/// It is a higher level interface over the actual filesystem on disk storage, i.e. `ext`, `zfs`,
/// `xfs`. It is focused on the miscellaneous readable resources.
///
/// All file system implementations must implement this trait.
pub trait FileSystem: Sync + Send + 'static {
  fn status(&self, path: &VFSPath) -> Result<ExitCode, ()>;

  /// Set the current working directory, aka `cd` command.
  fn set_cwd(&mut self, path: &VFSPath) -> ExitCode;

  /// Get the current working directory of this file system.
  fn get_cwd(&self, path: &VFSPath) -> Result<&VFSPath, ()>;

  fn exists(&self, path: &VFSPath) -> bool;

  fn get_real_path(&self, path: &VFSPath) -> Result<VFSPath, ()>;

  fn get_file(&self, path: &VFSPath) -> Result<Box<dyn FileObject>, ()>;

  /// Open the file object.
  fn open(&mut self, path: &VFSPath) -> Result<(), ()>;

  fn read_dir(
    &self,
    path: &VFSPath,
  ) -> VspResult<Box<dyn DirectoryEntryIterator<Item = DirectoryEntry>>>;

  fn create_dir(&self, path: &VFSPath) -> VspResult<()>;
}

/// A normal and default implementation of `VirtualFileSystem` refers to the real filesystem.
pub struct RealFileSystem {
  cwd:      VFSPath,
  real_cwd: PathBuf,
}

/// An associated function to construct a real filesystem instance which located at the current
/// working directory. It is used as a default behavior for obtaining the filesystem instance,
pub fn get_real_file_system() -> RealFileSystem {
  let real_cwd = std::env::current_dir().unwrap();
  let cwd = VFSPath::from(real_cwd.to_str().unwrap().to_string());
  RealFileSystem { cwd, real_cwd }
}

impl FileSystem for RealFileSystem {
  fn status(&self, path: &VFSPath) -> Result<ExitCode, ()> {
    todo!()
  }

  fn set_cwd(&mut self, path: &VFSPath) -> ExitCode {
    todo!()
  }

  fn get_cwd(&self, path: &VFSPath) -> Result<&VFSPath, ()> {
    todo!()
  }

  fn exists(&self, path: &VFSPath) -> bool {
    path.to_path_buf().exists()
  }

  fn get_real_path(&self, path: &VFSPath) -> Result<VFSPath, ()> {
    todo!()
  }

  fn get_file(&self, path: &VFSPath) -> Result<Box<dyn FileObject>, ()> {
    todo!()
  }

  fn open(&mut self, path: &VFSPath) -> Result<(), ()> {
    todo!()
  }

  fn read_dir(
    &self,
    path: &VFSPath,
  ) -> VspResult<Box<dyn DirectoryEntryIterator<Item = DirectoryEntry>>> {
    todo!()
  }

  fn create_dir(&self, path: &VFSPath) -> VspResult<()> {
    todo!()
  }
}

pub struct RealFileObject {
  file: File,
}

impl FileObject for RealFileObject {
  fn get_name(&self) -> Result<OsString, ()> {
    Err(())
  }

  fn close(&mut self) -> ExitCode {
    exitcode::EXIT_OK
  }

  fn read_to_string(&mut self, buf: &mut String) -> VspResult<()> {
    let res = &self.file.read_to_string(buf);
    match res {
      Ok(_) => Ok(()),
      Err(e) => Err(VspError::new(e.to_string())),
    }
  }
}
