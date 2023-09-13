use std::net::SocketAddr;

use clap::arg;
use clap::Args;
use vsp_error::VspResult;
use vsp_lsp::config::Configuration;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Run as a daemon
  #[arg(short, long)]
  pub daemon:              bool,
  /// Communicate with LSP server through the stdin and stdout
  #[arg(long, group = "type")]
  pub stdio:               bool,
  /// Communicate with LSP server through the given unix socket
  #[cfg(unix)]
  #[arg(long = "sock")]
  pub socket:              Option<String>,
  /// Communicate with LSP server through the given IP address.
  #[arg(long = "addr")]
  pub address:             Option<SocketAddr>,
  // /// Log file to record the log
  // #[arg(long)]
  // pub log_file:            Option<String>,
  /// Enable verbose mode
  #[arg(short, long)]
  pub verbose:             bool,
  /// Print configuration schema
  #[arg(short, long)]
  pub print_config_schema: bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    let config = convert_to_configuration(self);
    vsp_lsp::start_lsp_server(config)
  }
}

/// Converto the command line arguments to the configuration and fulfill the default values.
fn convert_to_configuration(arg: &mut CandidateArgument) -> Configuration {
  Configuration {
    daemon:              arg.daemon.to_owned(),
    stdio:               arg.stdio.to_owned(),
    socket:              arg.socket.to_owned(),
    address:             arg.address.to_owned(),
    verbose:             false,
    print_config_schema: false,
  }
}
