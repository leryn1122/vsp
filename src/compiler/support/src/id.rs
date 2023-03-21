use nanoid::nanoid;

/// <h1>NanoId</h1>
#[derive(Eq, PartialEq)]
pub struct NanoId {
  id: String,
}

impl NanoId {
  pub fn new() -> Self {
    Self { id: nanoid!() }
  }
}
