#[macro_export]
macro_rules! raise {
  ($msg:literal $(,)?) => {
    $crate::__private::must_use({
      let error = $crate::__private::format_err($crate::__private::format_args!($msg));
      error
    })
  };
  ($err:expr $(,)?) => {
    $crate::__private::must_use({
      // let error = match $err {
      //     error => (&error).anyhow_kind().new(error),
      // };
      let error = $crate::VspError::new($err)
      error
    })
  };
  ($fmt:expr, $($arg:tt)*) => {
    $crate::VspError::msg($crate::__private::format!($fmt, $($arg)*))
  };
}
