#[cfg(not(feature = "no-std"))]
pub(crate) use std::backtrace::Backtrace;
#[cfg(feature = "no-std")]
pub(crate) enum Backtrace {}

#[cfg(not(feature = "no-std"))]
macro_rules! backtrace {
  () => {
    Some(std::backtrace::Backtrace::capture())
  };
}

#[cfg(feature = "no-std")]
macro_rules! backtrace {
  () => {
    None
  };
}

#[cfg(not(feature = "no-std"))]
macro_rules! backtrace_if_absent {
  ($err:expr) => {
    // match ($err as &dyn
    // std::error::Error).downcast_ref::<std::backtrace::Backtrace>() {
    //   Some(_) => None,
    //   None => backtrace!(),
    // }
    backtrace!()
  };
}

#[cfg(feature = "no-std")]
macro_rules! backtrace_if_absent {
  ($err:expr) => {
    None
  };
}
