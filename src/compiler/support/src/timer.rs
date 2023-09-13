use std::borrow::Cow;

use crate::id::NanoId;

pub struct Timer {
  pub name:        NanoId,
  pub description: Cow<'static, str>,
  pub user_time:   usize,
  pub system_time: usize,
  pub running:     bool,
  pub triggered:   bool,
}

impl Default for Timer {
  fn default() -> Self {
    Self {
      name:        NanoId::create(),
      description: Cow::from(""),
      user_time:   0,
      system_time: 0,
      running:     false,
      triggered:   false,
    }
  }
}
