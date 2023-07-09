use core::fmt::Display;
use std::io::Write;

use vsp_error::VspError;
use vsp_error::VspResult;

#[allow(dead_code)]
pub struct Shell {
  output: ShellOutput,
  needs_clear: bool,
}

impl Shell {
  pub fn from_writable(out: Box<dyn Write>) -> Self {
    Self {
      output: ShellOutput::Write(out),
      needs_clear: false,
    }
  }

  pub fn out(&mut self) -> &mut dyn Write {
    self.output.stdout()
  }

  pub fn err(&mut self) -> &mut dyn Write {
    self.output.stderr()
  }

  pub fn print(&mut self, message: Option<&dyn Display>) -> VspResult<()> {
    self
      .output
      .stdout()
      .write_all(message.unwrap().to_string().as_bytes())
      .map_err(VspError::from)
  }
}

impl Default for Shell {
  fn default() -> Self {
    Self {
      output: ShellOutput::Stream {
        stdout: std::io::stdout(),
        stderr: std::io::stderr(),
        tty: false,
      },
      needs_clear: false,
    }
  }
}

pub enum ShellOutput {
  Write(Box<dyn Write>),
  Stream {
    stdout: std::io::Stdout,
    stderr: std::io::Stderr,
    tty: bool,
  },
}

impl ShellOutput {
  /// Get `stdout`.
  fn stdout(&mut self) -> &mut dyn Write {
    match *self {
      ShellOutput::Write(ref mut w) => w,
      ShellOutput::Stream { ref mut stdout, .. } => stdout,
    }
  }

  /// Get `stderr`.
  fn stderr(&mut self) -> &mut dyn Write {
    match *self {
      ShellOutput::Write(ref mut w) => w,
      ShellOutput::Stream { ref mut stderr, .. } => stderr,
    }
  }
}
