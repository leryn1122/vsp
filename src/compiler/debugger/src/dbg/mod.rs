use std::borrow::Cow;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use clap::ArgMatches;
use clap::Command;

use vsp_error::VspResult;
use vsp_support::debug_println;

/// The debugger instance.
pub struct DebuggerInstance {
  prompt: Cow<'static, str>,
  command: Command,
}

impl DebuggerInstance {
  pub fn set_prompt_string(&mut self, prompt: impl Into<String>) {
    self.prompt = Cow::from(prompt.into());
  }

  pub fn get_prompt(&self) -> &str {
    self.prompt.as_ref()
  }

  /// Core loop on the debugger instance.
  /// Accept the stdin line and invoke the debugger instruction.
  pub fn core_loop(&self) -> VspResult<()> {
    'main: loop {
      let line = self.readline().unwrap();
      let line = line.trim();
      if line.is_empty() {
        continue 'main;
      }
      debug_println!("Current debug instruction: {}", line);

      match self.eval(line) {
        Ok(true) => {
          break 'main;
        }
        Ok(false) => {}
        Err(err_msg) => {
          write!(stderr(), "{}", err_msg).expect("");
          stderr().flush().expect("");
        }
      }
    }
    Ok(())
  }

  /// Returns a boolean value indicating whether to quit the debugger and the error message.
  pub(crate) fn eval(&self, line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("[ERROR]: Invalid quoting, please check your input.")?;
    self.match_(args)
  }

  pub(crate) fn match_(&self, args: Vec<String>) -> Result<bool, String> {
    let matches = self.command.clone().try_get_matches_from(args);
    match matches {
      Ok(mut matches) => self.parse_argument(&mut matches),
      Err(e) => Err(format!("Unknown command: {}", e.to_string())),
    }
  }

  pub(crate) fn readline(&self) -> Result<String, String> {
    self.print_prompt();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).map_err(|e| e.to_string())?;
    Ok(buffer)
  }

  fn print_prompt(&self) {
    write!(stdout(), "{}", self.get_prompt()).expect("");
    stdout().flush().expect("");
  }

  pub fn parse_argument(&self, matches: &mut ArgMatches) -> Result<bool, String> {
    match matches.subcommand() {
      Some(("break", _matches)) => {
        println!("break")
      }
      Some(("quit", _matches)) => return Ok(true),
      Some((_, _matches)) => {}
      _ => unreachable!(),
    }
    Ok(false)
  }
}

impl Default for DebuggerInstance {
  fn default() -> Self {
    Self {
      prompt: Cow::from("> "),
      command: DebugInstructionProvider::construct_debug_instruction_set(),
    }
  }
}

pub struct DebugInstructionProvider;

impl DebugInstructionProvider {
  pub fn construct_debug_instruction_set() -> Command {
    Command::new("")
      .multicall(true)
      .subcommand_required(true)
      .disable_help_subcommand(true)
      .subcommand(Command::new("break").alias("b"))
      .subcommand(Command::new("backtrace").alias("bt"))
      .subcommand(Command::new("continue").alias("c"))
      .subcommand(Command::new("next").alias("n"))
      .subcommand(Command::new("run").alias("r"))
      .subcommand(Command::new("print").alias("p"))
      .subcommand(Command::new("quit").alias("q"))
  }
}
