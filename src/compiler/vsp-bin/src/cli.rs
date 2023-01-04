use crate::config::Config;

pub fn run(config: &mut Config) -> anyhow::Result<()> {
  config.execute_command()
}

pub fn exit_with_error(error: anyhow::Error) {
  eprintln!("{}", error);
  std::process::exit(1);
}
