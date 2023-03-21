use std::ffi::CStr;
use std::ffi::CString;

/// A struct refers to `passwd` included by `<pwd.h>` in `libc`.
///
/// ```C++
/// /* A record in the user database.  */
/// struct passwd
/// {
///   char *pw_name;    /* Username.  */
///   char *pw_passwd;  /* Hashed passphrase, if shadow database
///                                  not in use (see shadow.h).  */
///   __uid_t pw_uid;    /* User ID.  */
///   __gid_t pw_gid;    /* Group ID.  */
///   char *pw_gecos;    /* Real name.  */
///   char *pw_dir;      /* Home directory.  */
///   char *pw_shell;    /* Shell program.  */
/// };
/// ```
#[repr(C)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Passwd {
  /// Username
  pub pw_name: CString,
  /// User password
  pw_passwd:   CString,
  /// User ID
  pw_uid:      libc::uid_t,
  /// Group ID
  pw_gid:      libc::uid_t,
  /// User display name
  pw_gecos:    CString,
  /// Home directory
  pw_dir:      CString,
  /// User default shell program
  pw_shell:    CString,
}

impl Passwd {
  /// Convert `libc::passwd` into Rust FFI form.
  unsafe fn from_libc(passwd: &libc::passwd) -> Self {
    let libc::passwd {
      pw_name,
      pw_passwd,
      pw_uid,
      pw_gid,
      pw_gecos,
      pw_dir,
      pw_shell,
    } = *passwd;
    Self {
      pw_name: CStr::from_ptr(pw_name).to_owned(),
      pw_passwd: CStr::from_ptr(pw_passwd).to_owned(),
      pw_uid,
      pw_gid,
      pw_gecos: CStr::from_ptr(pw_gecos).to_owned(),
      pw_dir: CStr::from_ptr(pw_dir).to_owned(),
      pw_shell: CStr::from_ptr(pw_shell).to_owned(),
    }
  }

  fn use_functor<T>(
    key: T,
    functor: unsafe extern "C" fn(
      key: T,
      pwd: *mut libc::passwd,
      buf: *mut libc::c_char,
      buflen: libc::size_t,
      result: *mut *mut libc::passwd,
    ) -> libc::c_int,
  ) -> Option<Self>
  where
    T: Copy,
  {
    let mut pwd: libc::passwd = unsafe { std::mem::zeroed() };
    let mut buf = Vec::with_capacity(getpw_r_size_max());
    let mut result = std::ptr::null_mut();

    unsafe {
      functor(key, &mut pwd, buf.as_mut_ptr(), buf.capacity(), &mut result);
      result.as_ref().map(|p| Passwd::from_libc(p))
    }
  }

  /// Return `passwd` struct from given uid.
  pub fn from_uid(uid: libc::uid_t) -> Option<Self> {
    Self::use_functor(uid, libc::getpwuid_r)
  }

  /// Return `passwd` struct from given user name.
  pub fn from_name(username: &str) -> Option<Self> {
    let c_user = CString::new(username).unwrap();
    Self::use_functor(c_user.as_ptr(), libc::getpwnam_r)
  }
}

/// Return current user.
pub fn current_user() -> Option<Passwd> {
  Passwd::from_uid(unsafe { libc::geteuid() })
}

//noinspection SpellCheckingInspection
/// Call `libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX)`.
#[inline(always)]
fn getpw_r_size_max() -> usize {
  let res = unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) };
  libc::c_long::max(res, 1 << 9) as usize
}

// extern "system" {
//   /// Get user ID.
//   ///
//   /// ```C++
//   /// extern int getpwuid_r (__uid_t __uid,
//   ///            struct passwd *__restrict __resultbuf,
//   ///            char *__restrict __buffer, size_t __buflen,
//   ///            struct passwd **__restrict __result)
//   ///            __nonnull ((2, 3, 5));
//   /// ```
//   #[allow(unused)]
//   fn getpwuid_r(
//     uid: libc::uid_t,
//     pwd: *mut Passwd,
//     buf: *mut libc::c_char,
//     buflen: libc::size_t,
//     result: *mut *mut Passwd,
//   ) -> i32;
//
//   /// Get the effective user ID of the calling process.
//   ///
//   /// ```C++
//   /// /* Get the effective user ID of the calling process.  */
//   /// extern __uid_t geteuid (void) __THROW;
//   /// ```
//   #[allow(unused)]
//   fn geteuid() -> libc::uid_t;
// }

#[cfg(test)]
mod tests {
  #[test]
  pub fn test_get_passwd() {
    if let Some(passwd) = super::current_user() {
      println!("Username: {}", passwd.pw_name.to_str().unwrap());
    } else {
      panic!()
    }
  }
}
