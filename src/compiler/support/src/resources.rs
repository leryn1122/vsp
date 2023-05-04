#[macro_export]
macro_rules! resources {
  ($path:literal) => {
    concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $path)
  };
}

#[macro_export]
macro_rules! resources_str {
  ($path:literal) => {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $path))
  };
}

#[cfg(test)]
mod tests {
  #[test]
  pub fn test_resources_macro() {
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
