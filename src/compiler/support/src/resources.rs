/// Returns the absolute path from the given relative path under the `/resources/` in the project
/// directory.
#[macro_export]
macro_rules! resources {
  ($path:literal) => {
    concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $path)
  };
}

/// Returns the file content in bytes from the given relative path under the `/resources/` in the
/// project directory.
#[macro_export]
macro_rules! resources_bytes {
  ($path:literal) => {
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $path))
  };
}

/// Returns the file content from the given relative path under the `/resources/` in the project
/// directory.
#[macro_export]
macro_rules! resources_str {
  ($path:literal) => {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $path))
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_resources_macro() {
    assert_eq!(
      include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/",
        "lorem_ipsum.txt"
      )),
      include_str!(resources!("lorem_ipsum.txt"))
    );
  }
}
