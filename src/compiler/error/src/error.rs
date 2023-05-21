use std::backtrace::Backtrace;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use anyhow::anyhow;

pub struct Error(anyhow::Error);

impl Error {
  pub fn from<E>(error: E) -> Self
  where
    E: StdError + Send + Sync + 'static,
  {
    Self(anyhow!(error))
  }

  pub fn new(message: impl Into<String>) -> Self {
    Self(anyhow!(message.into()))
  }

  pub fn message(&self) -> String {
    self.0.to_string()
  }

  pub fn backtrace(&self) -> &Backtrace {
    self.0.backtrace()
  }
}

impl Debug for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.0)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message())
  }
}

impl StdError for Error {}
