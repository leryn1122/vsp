use target_lexicon::Triple;

pub struct TargetOptions {
  pub triplet: Triple,
}

impl Default for TargetOptions {
  fn default() -> Self {
    Self {
      triplet: Triple::host(),
    }
  }
}
