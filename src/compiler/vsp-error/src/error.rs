use crate::StdError;
use std::backtrace::Backtrace;

///
#[repr(C)]
pub(crate) struct ErrorImpl {
  backtrace: Option<Backtrace>,
  // object: Box<dyn StdError>,
}

impl ErrorImpl {
  // /// Get error backtrace.
  // pub(crate) fn backtrace(this: Ref<Self>) -> &Backtrace {
  //   this
  //     .deref()
  //     .backtrace
  //     .as_ref()
  //     .expect("Failed to capture backtrace.")
  // }

  pub(crate) fn construct(_error: impl StdError, backtrace: Option<Backtrace>) -> Self {
    Self {
      backtrace,
      // object: Box::new(error),
    }
  }
}
