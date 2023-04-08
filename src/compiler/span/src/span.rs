use core::cmp::Ordering;

/// Position where the character lies in the file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
  pub line:   usize,
  pub column: usize,
}

impl Position {
  pub fn create() -> Self {
    Self {
      line:   1,
      column: 1,
    }
  }

  pub fn create_at(line: usize, column: usize) -> Self {
    Self { line, column }
  }

  /// A start position.
  pub fn start() -> Self {
    Self::create_at(1, 1)
  }

  pub fn none() -> Self {
    Self::create_at(0, 0)
  }

  /// Carriage returns. Returns to the beginning of the line.
  pub fn carriage_return(&mut self) {
    self.column = 0;
  }

  /// Line feed. Returns to the beginning of the next line.
  pub fn line_feed(&mut self) {
    self.line += 1;
    self.column = 1;
  }

  pub fn forward(&mut self) {
    self.column += 1;
  }
}

impl PartialOrd for Position {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Position {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.line.cmp(&other.line) {
      Ordering::Equal => self.column.cmp(&other.column),
      Ordering::Greater => Ordering::Greater,
      Ordering::Less => Ordering::Less,
    }
  }
}

/// Code span, from the start to the end position within the file.
pub struct Span {
  pub start: Position,
  pub end:   Position,
}

impl Default for Span {
  fn default() -> Self {
    Self {
      start: Position::start(),
      end:   Position::start(),
    }
  }
}

impl Span {
  pub fn create(start: Position, end: Position) -> Self {
    Self {
      start: start,
      end:   end,
    }
  }

  /// Returns if the given position is within the given span.
  pub fn contain(&self, position: Position) -> bool {
    self.start.cmp(&position).is_gt() && self.end.cmp(&position).is_gt()
  }
}
