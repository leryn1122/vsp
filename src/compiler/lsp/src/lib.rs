use log::LevelFilter;
use lsp_server::Connection;
use lsp_server::IoThreads;
use lsp_types::InitializeParams;
use lsp_types::InitializeResult;
use lsp_types::ServerInfo;
use simplelog::ColorChoice;
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::SharedLogger;
use simplelog::TermLogger;
use simplelog::TerminalMode;

use vsp_error::VspError;
use vsp_error::VspResult;
use vsp_support::json::from_json;

use crate::config::Configuration;
use crate::server::server_capabilities;

pub mod config;
pub(crate) mod server;

/// Run language server.
pub fn start_server(config: Configuration) -> VspResult<()> {
  // Initialize.
  init_logger();

  log::trace!("Ready to run server: {}", env!("CARGO_PKG_VERSION"));

  let (connection, io_threads) = match connection_by_type(&config) {
    Ok(res) => res,
    Err(e) => return Err(VspError::from(e)),
  };
  let (initialize_id, initialize_params) = connection.initialize_start().unwrap();
  log::trace!("InitializeParams: {}", initialize_params);

  let initialize_params =
    from_json::<InitializeParams>("InitializeParams", &initialize_params).unwrap();

  let initialize_result = InitializeResult {
    capabilities: server_capabilities(),
    server_info: Some(ServerInfo {
      name: String::from(""),
      version: Some(String::from(env!("CARGO_PKG_VERSION"))),
    }),
  };
  let initialize_result = serde_json::to_value(initialize_result).unwrap();

  io_threads.join().unwrap();
  Ok(())
}

pub fn connection_by_type(config: &Configuration) -> std::io::Result<(Connection, IoThreads)> {
  if let Some(addr) = config.address {
    Connection::connect(addr)
  } else if let Some(addr) = &config.socket {
    Connection::connect(addr.as_str())
  } else if config.stdio {
    Ok(Connection::stdio())
  } else {
    Ok(Connection::stdio())
  }
}

pub fn init_logger() {
  let loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
    LevelFilter::Trace,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )];
  CombinedLogger::init(loggers).unwrap();
}
