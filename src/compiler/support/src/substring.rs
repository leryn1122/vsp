extern crate std as core;

/// Providing substring method for `str`.
pub trait Substring {
  fn substring(&self, start_index: usize, end_index: usize) -> &str;
}

impl Substring for str {
  #[must_use]
  #[allow(deprecated)]
  fn substring(&self, start_index: usize, end_index: usize) -> &str {
    if end_index <= start_index {
      return "";
    }

    let mut indices = self.char_indices();

    let obtain_index = |(index, _char)| index;
    let str_len = self.len();

    unsafe {
      self.slice_unchecked(
        indices.nth(start_index).map_or(str_len, obtain_index),
        indices.nth(end_index - start_index - 1).map_or(str_len, obtain_index),
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
