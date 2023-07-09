use core::cmp::Ordering;

/// Position where the character lies in the file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

impl Position {
  pub fn new() -> Self {
    Self { line: 1, column: 1 }
  }

  pub fn at(line: usize, column: usize) -> Self {
    Self { line, column }
  }

  /// A start position.
  pub fn start() -> Self {
    Self::at(1, 1)
  }

  pub fn none() -> Self {
    Self::at(0, 0)
  }

  /// Carriage returns. Returns to the beginning of the line.
  pub fn carriage_return(&mut self) -> Self {
    self.column = 1;
    *self
  }

  /// Line feed. Returns to the beginning of the next line.
  pub fn line_feed(&mut self) -> Self {
    self.line += 1;
    self.column = 1;
    *self
  }

  pub fn forward(&mut self) -> Self {
    self.column += 1;
    *self
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

impl Default for Position {
  fn default() -> Self {
    Self::new()
  }
}

/// It is typically represented as a pair of start and end positions, where each location is a tuple
/// of line and column numbers.
/// It is useful for operations such as code highlighting, error reporting, and code generation,
/// where it is necessary to work with a range of characters rather than just a single point in the
/// source code.
///
/// Construct the span:
///
/// ```rust
/// use vsp_span::span::Position;
/// use vsp_span::span::Span;
///
/// let start = Position::at(0, 0);
/// let end = Position::at(0, 12);
/// let span = Span::range(start, end);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Span {
  pub start: Position,
  pub end: Position,
}

impl Default for Span {
  fn default() -> Self {
    Self {
      start: Position::start(),
      end: Position::start(),
    }
  }
}

impl Span {
  pub fn range(start: Position, end: Position) -> Self {
    Self { start, end }
  }

  #[allow(clippy::clone_on_copy)]
  pub fn single_char(pos: Position) -> Self {
    Self {
      start: pos,
      end: pos,
    }
  }

  /// Returns if the given position is within the given span.
  pub fn contain(&self, position: Position) -> bool {
    self.start.cmp(&position).is_gt() && self.end.cmp(&position).is_gt()
  }

  pub fn expand(&self) -> (usize, usize, usize, usize) {
    (
      self.start.line,
      self.start.column,
      self.end.line,
      self.end.column,
    )
  }

  pub fn expand_as_array(&self) -> [usize; 4] {
    [
      self.start.line,
      self.start.column,
      self.end.line,
      self.end.column,
    ]
  }
}
