use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Output;

/// Builder for external process invocation instead of `std::process::Command`.
#[derive(Clone, Debug)]
pub struct ProcessBuilder {
  /// The main command to execute.
  cmd:  OsString,
  /// The current working directory where the command run.
  cwd:  Option<OsString>,
  /// Argument lists passed to the command.
  args: Vec<OsString>,
  /// Environmental variables used by the command.
  env:  BTreeMap<String, Option<OsString>>,
}

impl Display for ProcessBuilder {
  #[allow(unused_variables)]
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl ProcessBuilder {
  pub fn new<T: AsRef<OsStr>>(cmd: T) -> Self {
    Self {
      cmd:  cmd.as_ref().to_os_string(),
      cwd:  None,
      args: Vec::new(),
      env:  BTreeMap::new(),
    }
  }

  pub fn cwd<T: AsRef<OsStr>>(&mut self, cwd: T) -> &mut Self {
    self.cwd = Some(cwd.as_ref().to_os_string());
    self
  }

  pub fn arg<T: AsRef<OsStr>>(&mut self, arg: T) -> &mut Self {
    self.args.push(arg.as_ref().to_os_string());
    self
  }

  pub fn status(&self) -> std::io::Result<ExitStatus> {
    let mut cmd = self.build_command();
    cmd.spawn()?.wait()
  }

  pub fn exec(&self) -> anyhow::Result<()> {
    let exit = self.status()?;
    if exit.success() {
      Ok(())
    } else {
      Err(ProcessError::new(Some(exit), &format!("fail to run: {}", self), None).into())
    }
  }

  fn build_command(&self) -> Command {
    let mut command = Command::new(&self.cmd);
    if let Some(cwd) = self.get_cwd() {
      command.current_dir(cwd);
    }

    // Iteratively set the environmental variables.
    for (k, v) in &self.env {
      match *v {
        Some(ref v) => {
          command.env(k, v);
        }
        None => {
          command.env_remove(k);
        }
      }
    }

    // Iteratively pass the arguments to the command.
    for arg in &self.args {
      command.arg(arg);
    }
    command
  }

  pub fn get_cwd(&self) -> Option<&Path> {
    self.cwd.as_ref().map(Path::new)
  }
}

#[derive(Debug)]
pub struct ProcessError {
  pub code:    Option<i32>,
  pub message: String,
}

impl Display for ProcessError {
  #[allow(unused_variables)]
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl std::error::Error for ProcessError {}

impl ProcessError {
  #[allow(unused_variables)]
  pub fn new(status: Option<ExitStatus>, message: &str, output: Option<&Output>) -> ProcessError {
    Self {
      message: message.to_string(),
      code:    status.and_then(|s| s.code()),
    }
  }
}
