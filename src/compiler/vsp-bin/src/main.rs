use config::Config;

mod cli;
mod config;
mod ops;

fn main() {
  let mut config = Config::default();
  let result = cli::run(&mut config);

  match result {
    Ok(_) => {}
    Err(e) => {
      cli::exit_with_error(e);
    }
  }
}
