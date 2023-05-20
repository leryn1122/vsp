use clap::arg;
use clap::Args;
use vsp_lsp::run_server;

use crate::ops::Entrypoint;

#[derive(Args)]
pub struct CandidateArgument {
  /// Run as a daemon
  #[arg(short, long)]
  daemon:              bool,
  /// Communicate with LSP server through the stdin and stdout
  #[arg(long)]
  stdio:               bool,
  /// Communicate with LSP server through the given unix socket
  #[cfg(unix)]
  #[arg(long)]
  socket:              Option<String>,
  /// Log file to record the log
  #[arg(long)]
  log_file:            Option<String>,
  /// Enable verbose mode
  #[arg(short, long)]
  verbose:             bool,
  /// Print configuration schema
  #[arg(short, long)]
  print_config_schema: bool,
}

impl Entrypoint for CandidateArgument {
  fn entrypoint(&self) -> anyhow::Result<()> {
    let _socket = self.socket.as_deref().unwrap_or(&"unix:///tmp/vsplsp.sock".to_string());

    let _ = run_server();

    Ok(())
  }
}
