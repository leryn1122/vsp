/// Low-level source file VFS.
pub trait SourceFileManager {
  fn contains(location: impl Location, file_obj: impl FileObject) -> bool;
}

/// File object
pub trait FileObject {
  /// Get source file
  fn get_kind() -> Kind;
}

pub enum Kind {
  /// Source file with extension `.vsp`.
  Source,
  /// Other source file kind.
  Other,
}

/// Location of file objects where to find the source file.
pub trait Location {}

/// Position in the plain text file: tuple of row number and column number.
/// It is used to locate the source code error, etc.
#[derive(Debug)]
pub struct Position {
  line: usize,
  column: usize,
}

impl Position {
  /// Return a position for none. None reaches there.
  pub fn none() -> Self {
    Self { line: 0, column: 0 }
  }

  /// Go to the begin of next line.
  pub fn next_line(&mut self) {
    self.line = self.line + 1;
    self.column = 1;
  }

  /// Go to the next column.
  pub fn next_column(&mut self) {
    self.column = self.column + 1;
  }
}

impl Default for Position {
  /// Return the start position in plain text file.
  fn default() -> Self {
    Self { line: 1, column: 1 }
  }
}

impl PartialEq<Self> for Position {
  fn eq(&self, other: &Self) -> bool {
    self.line == other.line && self.column == self.column
  }
}

impl Eq for Position {}
