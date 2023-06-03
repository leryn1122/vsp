use std::borrow::Cow;

use getset::Getters;
use getset::Setters;
use semver::Version;
use target_lexicon::Triple;

/// Target options for compilation.
#[derive(Getters, Setters)]
pub struct TargetOptions {
  /// Semantic version for target artifact.
  #[getset(get, set)]
  version:       Version,
  #[getset(get = "pub", set = "pub")]
  binary:        Option<Cow<'static, str>>,
  #[getset(get = "pub", set = "pub")]
  library:       bool,
  /// Host triple
  #[getset(get = "pub", set = "pub")]
  host_triple:   Triple,
  /// Target triple
  #[getset(get = "pub", set = "pub")]
  target_triple: Triple,
  /// Optimization level
  #[getset(get = "pub", set = "pub")]
  optimization:  u8,
}

impl TargetOptions {
  #[cfg(debug_assertions)]
  pub fn debug_print_status(&self) {
    println!("Version = {}", &self.version);
    println!("Binary = {}", &self.binary.clone().unwrap_or(Cow::from("")));
    println!("Library = {}", &self.library);
    println!("Host triple = {}", &self.host_triple);
    println!("Target triple = {}", &self.target_triple);
  }
}

impl Default for TargetOptions {
  fn default() -> Self {
    Self {
      version:       Version::new(0, 0, 0),
      binary:        None,
      library:       false,
      host_triple:   Triple::host(),
      target_triple: Triple::host(),
      optimization:  3,
    }
  }
}

pub struct LangOptions {}

impl Default for LangOptions {
  fn default() -> Self {
    Self {}
  }
}
