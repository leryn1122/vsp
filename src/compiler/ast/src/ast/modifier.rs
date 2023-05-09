/// Definite of constancy referring to the preserved word `const`.
#[derive(Debug)]
pub enum Constancy {
  Constant,
  None,
}

impl Constancy {
  pub fn is_constant(&self) -> bool {
    match self {
      Constancy::Constant => true,
      Constancy::None => false,
    }
  }
}

/// Accessibility.
/// - `Public`
/// - `Private`
pub enum Accessibility {
  Public,
  Private,
}
