use std::io::stdout;
use std::io::Write;

use clap::Command;

pub fn do_run_repl() -> anyhow::Result<()> {
  let repl = REPL::new();
  match repl.core_loop() {
    Ok(res) => Ok(res),
    Err(message) => Err(anyhow::Error::msg(message)),
  }
}

#[allow(non_snake_case)]
pub struct REPL {
  prompt: String,
}

impl REPL {
  pub fn new() -> Self {
    Self {
      prompt: "> ".to_string(),
    }
  }

  pub fn set_prompt_string(&mut self, prompt: impl Into<String>) {
    self.prompt = prompt.into();
  }

  #[allow(unused_must_use)]
  pub fn core_loop(&self) -> anyhow::Result<(), String> {
    loop {
      let line = readline().unwrap();
      let line = line.trim();
      if line.is_empty() {
        continue;
      }

      match self.eval(line) {
        Ok(quit) => {
          if quit {
            break;
          }
        }
        Err(err) => {
          write!(std::io::stdout(), "{}", err).map_err(|e| e.to_string());
          std::io::stdout().flush().map_err(|e| e.to_string());
        }
      }
    }

    Ok(())
  }

  pub fn run() -> Result<(), ()> {
    Ok(())
  }

  #[allow(unused_must_use)]
  pub fn eval(&self, line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("[ERROR]: Invalid quoting, please check your input.")?;
    let matches = cli().try_get_matches_from(args).map_err(|e| e.to_string())?;
    match matches.subcommand() {
      Some(("ping", _matches)) => {
        write!(std::io::stdout(), "Pong").map_err(|e| e.to_string());
        std::io::stdout().flush().map_err(|e| e.to_string())?;
      }
      Some(("quit", _matches)) => {
        write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string());
        std::io::stdout().flush().map_err(|e| e.to_string())?;
        return Ok(true);
      }
      Some((name, _matches)) => unimplemented!("{}", name),
      None => unreachable!("subcommand required"),
    }

    Ok(false)
  }
}

impl Default for REPL {
  fn default() -> Self {
    Self::new()
  }
}

fn cli() -> Command {
  // strip out usage
  const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
  // strip out name/version
  const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

  Command::new("repl")
    .multicall(true)
    .arg_required_else_help(true)
    .subcommand_required(true)
    .subcommand_value_name("APPLET")
    .subcommand_help_heading("APPLETS")
    .help_template(PARSER_TEMPLATE)
    .subcommand(Command::new("ping").about("Get a response").help_template(APPLET_TEMPLATE))
    .subcommand(
      Command::new("quit")
        .alias("exit")
        .about("Quit the REPL")
        .help_template(APPLET_TEMPLATE),
    )
}

#[allow(unused_must_use)]
pub(crate) fn readline() -> Result<String, String> {
  write!(stdout(), "$ ").map_err(|e| e.to_string());
  stdout().flush().map_err(|e| e.to_string())?;
  let mut buffer = String::new();
  std::io::stdin().read_line(&mut buffer).map_err(|e| e.to_string())?;
  Ok(buffer)
}
