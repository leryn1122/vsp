#[macro_use]
mod backtrace;
mod chain;
mod error;
mod fmt;
mod ptr;
mod wrapper;
#[macro_use]
pub mod macros;

use crate::error::ErrorImpl;

#[cfg(feature = "no-std")]
trait StdError: Debug + Display {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    None
  }
}

#[cfg(not(feature = "no-std"))]
use std::error::Error as StdError;

/// Use `VspResult<()>` as the alias of `core::result::Result<(), E>`
///
/// ```rust
/// use vsp_error::VspResult;
/// ```
pub type VspResult<T = ()> = core::result::Result<T, VspError>;

/// Use `VspError` as the alias of `core::result::Result::Err(err)`
///
/// ```rust
/// use vsp_error::VspError;
/// ```
#[repr(transparent)]
pub struct VspError {
  ptr: crate::ptr::Own<ErrorImpl>,
}

/// Equivalent to `core::result::Result::Ok(t)` in `vsp_error` framework.
#[allow(non_snake_case)]
pub fn Ok<T>(t: T) -> VspResult<T> {
  Result::Ok(t)
}

#[derive(Clone)]
#[cfg(not(feature = "no-std"))]
pub struct Chain<'a> {
  state: crate::chain::ChainState<'a>,
}

#[doc(hidden)]
pub mod __private {
  extern crate alloc;
  pub use alloc::fmt;
  pub use alloc::format;
  pub use core::concat;
  pub use core::fmt::Arguments;
  pub use core::format_args;
  pub use core::stringify;

  use crate::VspError;

  #[doc(hidden)]
  #[inline]
  pub fn format_err(args: Arguments) -> VspError {
    let fmt_arguments_as_str = args.as_str();

    if let Some(message) = fmt_arguments_as_str {
      // anyhow!("literal"), can downcast to &'static str
      VspError::msg(message)
    } else {
      // anyhow!("interpolate {var}"), can downcast to String
      VspError::msg(fmt::format(args))
    }
  }

  #[doc(hidden)]
  #[inline]
  #[must_use]
  pub fn must_use(error: VspError) -> VspError {
    error
  }
}
