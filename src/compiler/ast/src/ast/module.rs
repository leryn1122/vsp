//! Module system
//!
//!
//! ```plaintext
//! ```

use std::borrow::Cow;

///
pub struct Path {
  segments: Vec<Cow<'static, str>>,
}

/// # Package
pub struct Package {
  pub name:   String,
  pub author: String,
}

/// # Module
pub struct Module {}
