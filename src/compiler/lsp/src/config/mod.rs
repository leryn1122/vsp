use std::net::SocketAddr;

pub struct Configuration {
  pub daemon:              bool,
  pub stdio:               bool,
  #[cfg(unix)]
  pub socket:              Option<String>,
  pub address:             Option<SocketAddr>,
  pub log_file:            Option<String>,
  pub verbose:             bool,
  pub print_config_schema: bool,
}
