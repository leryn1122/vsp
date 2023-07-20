use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use crate::support::compression::create_tar_archive_by_dir;
use crate::support::compression::create_zip_archive_by_dir;

fn to_path(str: impl Into<OsString>, cwd: &PathBuf) -> PathBuf {
  let path = PathBuf::from(str.into());
  if path.is_absolute() {
    path
  } else {
    cwd.join(path)
  }
}

/// ```bash
/// cd <target>
/// ```
pub fn cd<'a>(_cwd: &'a PathBuf, target: &'a PathBuf) -> std::io::Result<&'a PathBuf> {
  std::env::set_current_dir(target).map(|_| target)
}

pub fn cp(
  cwd: &PathBuf,
  src: impl Into<OsString>,
  target: impl Into<OsString>,
) -> std::io::Result<u64> {
  let src = to_path(src, &cwd);
  let target = to_path(target, &cwd);
  let target = if !target.is_dir() {
    target
  } else {
    target.join(&src.file_name().unwrap())
  };
  if !target.exists() {
    std::fs::copy(&src, target)
  } else {
    Ok(0)
  }
}

/// ```bash
/// mkdir -p <dir>
/// ```
pub fn mkdir(cwd: &PathBuf, dir: impl Into<OsString>) -> std::io::Result<PathBuf> {
  let res = to_path(dir, cwd);
  if !res.exists() {
    std::fs::create_dir(&res).map(|_| res)
  } else {
    Ok(res)
  }
}

/// ```bash
/// mkdir -p <dir>
/// ```
pub fn mkdir_p(cwd: &PathBuf, dir: impl Into<OsString>) -> std::io::Result<PathBuf> {
  let res = to_path(dir, cwd);
  std::fs::create_dir_all(&res).map(|_| res)
}

///
pub fn ln_s<P>(cwd: &PathBuf, src: P, target: P) -> std::io::Result<()>
where
  P: AsRef<Path>,
{
  let old_cwd = std::env::current_dir().unwrap();
  std::env::set_current_dir(&cwd).unwrap();

  #[cfg(unix)]
  let res = std::os::unix::fs::symlink(src, target);
  #[cfg(target_family = "windows")]
  let res = std::os::windows::fs::rename(source, target);

  std::env::set_current_dir(&old_cwd).unwrap();
  res
}

pub fn tar(
  cwd: &PathBuf,
  file: impl Into<OsString>,
  dir: impl Into<OsString>,
) -> std::io::Result<()> {
  let old_cwd = std::env::current_dir().unwrap();
  std::env::set_current_dir(&cwd).unwrap();

  let res = create_tar_archive_by_dir(&PathBuf::from(file.into()), &PathBuf::from(dir.into()));

  std::env::set_current_dir(&old_cwd).unwrap();
  res
}

pub fn zip(
  cwd: &PathBuf,
  file: impl Into<OsString>,
  dir: impl Into<OsString>,
) -> std::io::Result<()> {
  let old_cwd = std::env::current_dir().unwrap();
  std::env::set_current_dir(&cwd).unwrap();

  let res = create_zip_archive_by_dir(&PathBuf::from(file.into()), &PathBuf::from(dir.into()));

  std::env::set_current_dir(&old_cwd).unwrap();
  res
}
