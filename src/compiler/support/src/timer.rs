use std::borrow::Cow;

pub struct Timer {
  name:        Cow<'static, str>,
  description: Cow<'static, str>,
  user_time:   usize,
  system_time: usize,
  running:     bool,
  triggered:   bool,
}

impl Default for Timer {
  fn default() -> Self {
    Self {
      name:        Cow::from(""),
      description: Cow::from(""),
      user_time:   0,
      system_time: 0,
      running:     false,
      triggered:   false,
    }
  }
}
