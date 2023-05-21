pub mod path;

use std::ffi::OsString;

use getset::Getters;
use getset::MutGetters;
use vsp_error::VspResult;
use vsp_support::exitcode::ExitCode;

use crate::vfs::path::VFSPath;

/// Abstract file object trait for the virtual filesystem.
pub trait File {
  fn get_name(&self) -> Result<OsString, ()>;

  fn close(&mut self) -> ExitCode;
}

/// File type for file objects on filesystem.
#[derive(PartialEq)]
pub enum FileType {
  RegularFile,
  DirectoryFile,
  #[cfg(unix)]
  SymbolicLink,
  #[cfg(unix)]
  SocketFile,
  FileNotFound,
  Unknown,
}

enum AccessMode {
  Read,
  Write,
  Execute,
}

/// File metadata type for file objects on virtual filesystem.
pub struct Metadata {
  name:       OsString,
  uid:        u32,
  gid:        u32,
  size:       u64,
  file_type:  FileType,
  permission: (),
}

impl Metadata {
  pub fn exists(&self) -> bool {
    ![FileType::FileNotFound, FileType::Unknown].contains(&self.file_type)
  }

  pub fn is_directory(&self) -> bool {
    self.file_type == FileType::DirectoryFile
  }

  pub fn is_regular_file(&self) -> bool {
    self.file_type == FileType::RegularFile
  }

  pub fn is_symlink(&self) -> bool {
    self.file_type == FileType::SymbolicLink
  }

  pub fn is_other(&self) -> bool {
    ![FileType::RegularFile, FileType::DirectoryFile].contains(&self.file_type)
  }

  pub fn is_unknown(&self) -> bool {
    self.file_type == FileType::Unknown
  }
}

/// A member of a directory, used for `DirectoryIterator`, carries the necessary information for all
/// the file system implementations.
#[derive(Getters, MutGetters)]
pub struct DirectoryEntry {
  #[getset(get)]
  path:      VFSPath,
  #[getset(get)]
  file_type: FileType,
  // #[getset(get)]
  // follow_symlinks: bool,
}

impl DirectoryEntry {
  fn new(path: VFSPath, file_type: FileType) -> Self {
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

/// # Virtual File System
/// All file system implementations must implement this trait.
pub trait VirtualFileSystem: Sync + Send + 'static {
  fn status(path: &VFSPath) -> Result<ExitCode, ()>;

  /// Set the current working directory, aka `cd` command.
  fn set_cwd(&mut self, path: &VFSPath) -> ExitCode;

  /// Get the current working directory of this file system.
  fn get_cwd(&self, path: &VFSPath) -> Result<&VFSPath, ()>;

  fn exists(path: &VFSPath) -> bool;

  fn get_real_path(path: &VFSPath) -> Result<VFSPath, ()>;

  /// Open the file object.
  fn open<File>(path: &VFSPath) -> Result<File, ()>
  where
    File: crate::vfs::File;

  fn read_dir<Dirs>(&self, path: &VFSPath) -> VspResult<Dirs>
  where
    Dirs: DirectoryEntryIterator;

  fn create_dir(&self, path: &VFSPath) -> VspResult<()>;
}

/// A normal and default implementation of `VirtualFileSystem` refers to the real filesystem.
pub struct RealFileSystem {
  // cwd: VFSPath,
}

impl RealFileSystem {}
