use std::fmt::{Display, Formatter};

use smallvec::SmallVec;

#[allow(dead_code)]
pub struct Table {
  /// True if show table header.
  show_header: bool,
  /// Table column delimiter.
  column_separator: &'static str,
  /// Table row delimiter.
  row_separator: &'static str,
  /// **Note:** It is not recommended to display more than 8-column table in terminal.
  columns: SmallVec<[Column; 8]>,
  /// Null default value.
  null_default: &'static str,
}

impl Table {
  #[allow(dead_code)]
  pub fn create(headers: Vec<impl Into<String>>) -> Self {
    let mut table = Self::default();

    let mut columns = SmallVec::new();
    for header in headers {
      let column = Column::from(header);
      columns.push(column);
    }

    table.columns = columns;
    table
  }

  #[allow(dead_code)]
  pub fn add_column<T>(&mut self, _t: T) -> &mut Self {
    self
  }

  pub fn add_row<T>(&mut self, _t: T) -> &mut Self {
    self
  }
}

impl Default for Table {
  fn default() -> Self {
    Self {
      show_header: true,
      column_separator: " ",
      row_separator: "\n",
      columns: SmallVec::new(),
      null_default: "null",
    }
  }
}

impl Display for Table {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.show_header {
      let headers: Vec<String> = self
        .columns
        .to_vec()
        .iter()
        .map(|c| c.header.clone())
        .collect();
      let mut iter = headers.iter();
      while let Some(header) = iter.next() {
        f.write_str(header)?;
      }
    }

    Ok(())
  }
}

#[derive(Clone)]
#[allow(dead_code)]
struct Column {
  /// Column header.
  header: String,
  /// Column alignment.
  align: Alignment,
  /// Column max width.
  width: usize,
}

impl Column {
  fn from(header: impl Into<String>) -> Self {
    Self {
      header: header.into(),
      align: Alignment::Left,
      width: 0,
    }
  }
}

/// Column alignment.
#[allow(dead_code)]
#[derive(Clone)]
enum Alignment {
  Justify,
  Left,
  Middle,
  Right,
}
