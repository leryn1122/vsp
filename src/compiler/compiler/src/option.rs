use target_lexicon::Triple;

pub struct TargetOptions {
  pub host_triplet:   Triple,
  pub target_triplet: Triple,
}

impl Default for TargetOptions {
  fn default() -> Self {
    Self {
      host_triplet:   Triple::host(),
      target_triplet: Triple::host(),
    }
  }
}
