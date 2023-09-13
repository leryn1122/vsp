extern crate std as core;

/// Providing substring method for `str`.
pub trait Substring {
  fn substring(&self, start_index: usize, end_index: usize) -> &str;
}

impl Substring for str {
  #[must_use]
  #[allow(deprecated)]
  fn substring(&self, start: usize, end: usize) -> &str {
    if end <= start {
      return "";
    }

    let mut indices = self.char_indices();

    let obtain = |(index, _char)| index;
    let len = self.len();

    unsafe {
      self.slice_unchecked(
        indices.nth(start).map_or(len, obtain),
        indices.nth(end - start - 1).map_or(len, obtain),
      )
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_substring() {
    assert_eq!("foobar".substring(0, 3), "foo");
  }

  #[test]
  fn test_out_of_bounds() {
    assert_eq!("foobar".substring(0, 10), "foobar");
    assert_eq!("foobar".substring(6, 10), "");
  }

  #[test]
  fn test_start_less_than_end() {
    assert_eq!("foobar".substring(3, 2), "");
  }

  #[test]
  fn test_start_and_end_equal() {
    assert_eq!("foobar".substring(3, 3), "");
  }
}
