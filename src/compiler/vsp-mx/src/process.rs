/// VM Process model
pub struct VMProcess {
  /// PID
  pub(crate) pid: u16,
  /// Start command.
  pub(crate) cmd: String,
}

impl VMProcess {
  pub fn from(pid: u16, cmd: String) -> Self {
    Self { pid, cmd }
  }

  pub fn from_anonymous(pid: u16) -> Self {
    Self::from(pid, "[anonymous]".to_string())
  }

  pub fn get_pid(&self) -> u16 {
    self.pid
  }

  pub fn get_cmd(&self) -> &String {
    &self.cmd
  }
}

/// Entry point to list the all VM processes.
/// The implementation is inspired by Java Hot Spot performance data.
pub fn list_all_vm_processes() -> anyhow::Result<Vec<VMProcess>> {
  do_list_all_vm_processes()
}

#[cfg(target_os = "linux")]
pub const PFILE_DIR_PREFIX: &str = "vsproc.d";

#[cfg(target_os = "linux")]
pub fn do_list_all_vm_processes() -> anyhow::Result<Vec<VMProcess>> {
  use vsp_platform::linux::os;
  let passwd = os::current_user().unwrap();
  let username: &str = passwd.pw_name.to_str().unwrap();

  let mut vm_procs = Vec::new();

  let pfile_dir = std::env::temp_dir().join(PFILE_DIR_PREFIX).join(username);
  if !pfile_dir.exists() {
    return Ok(vm_procs);
  }
  for entry in std::fs::read_dir(pfile_dir).unwrap() {
    let entry = entry.unwrap();
    let filename = entry.file_name();
    let pid: u16 = filename.into_string().unwrap().parse().unwrap();
    let vm_proc = VMProcess::from_anonymous(pid);
    vm_procs.push(vm_proc);
  }
  Ok(vm_procs)
}

#[cfg(target_os = "windows")]
pub fn do_list_all_vm_processes() -> anyhow::Result<Vec<VMProcess>> {
  Ok(Vec::new())
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::*;

  #[test]
  fn test_list_all_vm_processes() {
    let res = super::list_all_vm_processes();
  }
}
