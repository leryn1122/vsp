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
