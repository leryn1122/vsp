/// All came from `sysexits.h`
///
/// ```
/// extern crate vsp_support;
/// use vsp_support::exitcode::ExitCode;
/// use vsp_support::exitcode::EXIT_OK;
///
/// std::process::exit(EXIT_OK);
/// ```
pub type ExitCode = i32;

pub const EXIT_OK: ExitCode = 0;
pub const EXIT__BASE: ExitCode = 64;
pub const EXIT_USAGE: ExitCode = 64;
pub const EXIT_DATAERR: ExitCode = 65;
pub const EXIT_NOINPUT: ExitCode = 66;
pub const EXIT_NOUSER: ExitCode = 67;
pub const EXIT_NOHOST: ExitCode = 68;
pub const EXIT_UNAVAILABLE: ExitCode = 69;
pub const EXIT_SOFTWARE: ExitCode = 70;
pub const EXIT_OSERR: ExitCode = 71;
pub const EXIT_OSFILE: ExitCode = 72;
pub const EXIT_CANTCREAT: ExitCode = 73;
pub const EXIT_IOERR: ExitCode = 74;
pub const EXIT_TEMPFAIL: ExitCode = 75;
pub const EXIT_PROTOCOL: ExitCode = 76;
pub const EXIT_NOPERM: ExitCode = 77;
pub const EXIT_CONFIG: ExitCode = 78;
// pub const EXIT_EX__MAX: ExitCode = 78;

/// Exit status code used for failures.
pub const EXIT_FAILURE: ExitCode = 1;

pub fn is_success(code: ExitCode) -> bool {
  code == EXIT_OK
}

pub fn is_error(code: ExitCode) -> bool {
  !is_success(code)
}
