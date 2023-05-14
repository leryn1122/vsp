use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use getset::Getters;
use getset::MutGetters;
use vsp_support::exitcode::ExitCode;

pub enum FileType {
  RegularFile,
  DirectoryFile,
  #[cfg(unix)]
  SocketFile,
  Unknown,
}

/// Abstract file trait for the virtual filesystem trait.
pub trait File {
  fn get_name(&self) -> Result<OsString, ()>;

  fn close(&mut self) -> ExitCode;
}

/// A member of a directory, used for `DirectoryIterator`, carries the necessary information for all
/// the file system implementations.
#[derive(Getters, MutGetters)]
pub struct DirectoryEntry {
  #[getset(get)]
  path:      PathBuf,
  #[getset(get)]
  file_type: FileType,
}

impl DirectoryEntry {
  fn new(path: &Path, file_type: FileType) -> Self {
    Self {
      path: path.to_path_buf(),
      file_type,
    }
  }
}

///
pub trait DirectoryEntryIterator: Iterator<Item = DirectoryEntry> {
  type Item = DirectoryEntry;
}

pub(crate) struct DirectoryEntryIteratorImpl;

impl Iterator for DirectoryEntryIteratorImpl {
  type Item = DirectoryEntry;

  fn next(&mut self) -> Option<Self::Item> {
    todo!()
  }
}

impl DirectoryEntryIterator for DirectoryEntryIteratorImpl {}

/// # Virtual File System
pub trait VirtualFileSystem {
  fn status(path: &Path) -> Result<ExitCode, ()>;

  fn open<File>(path: &Path) -> Result<File, ()>
  where
    File: crate::vfs::File;
}

pub struct DefaultFileSystem;

pub struct InMemoryFileSystem;
