/// Alias for `DiagnosticLevel`.
pub(crate) type Level = DiagnosticLevel;

pub enum DiagnosticLevel {
  Ignored,
  Note,
  Remark,
  Warning,
  Fatal,
}

pub struct DiagnosticConsumer {}

pub struct DiagnosticEngine {
  suppressed: bool,
  show_color: bool,
  level:      Level,
  owner:      DiagnosticConsumer,
}

impl DiagnosticEngine {}

impl Default for DiagnosticEngine {
  fn default() -> Self {
    Self {
      suppressed: false,
      show_color: false,
      level:      Level::Remark,
      owner:      DiagnosticConsumer {},
    }
  }
}
