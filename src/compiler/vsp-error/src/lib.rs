mod error;

use crate::error::ErrorImpl;
use std::backtrace::Backtrace;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};

/// Use `VspResult<()>` as `core::result::Result<(), E>`
///
/// ```rust
/// use vsp_error::VspResult;
/// ```
pub type VspResult<T = ()> = core::result::Result<T, VspError>;

/// Use `VspError` as `core::result::Result::Err(err)`,
///
/// ```rust
/// use vsp_error::VspError;
/// ```
#[repr(transparent)]
pub struct VspError {
  ptr: Box<ErrorImpl>,
}

impl VspError {
  /// Construct from the stdlib error.
  pub fn new<E>(error: E) -> Self
  where
    E: StdError + Send + Sync + 'static,
  {
    let error_impl = ErrorImpl::construct(error, Some(Backtrace::capture()));

    Self {
      ptr: Box::new(error_impl),
    }
  }

  // /// If the error is of given type.
  // pub fn is<E: StdError>(&self) -> bool {
  //   self.type_id() == TypeId::of::<E>()
  // }
  //
  // pub fn downcast<E: StdError>(&self) -> Option<&E> {
  //   if self.is::<E>() {
  //     // unsafe { Some(core::mem::transmute()) }
  //     None
  //   } else {
  //     None
  //   }
  // }
}

impl StdError for VspError {}

impl Debug for VspError {
  fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl Display for VspError {
  fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

/// Equivalent to `core::result::Result::Ok(t)` in `vsp_error` framework.
#[allow(non_snake_case)]
pub fn Ok<T>(t: T) -> VspResult<T> {
  Result::Ok(t)
}
