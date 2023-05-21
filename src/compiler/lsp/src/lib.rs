pub(crate) mod config;
pub(crate) mod server;

use log::info;
use log::trace;
use log::LevelFilter;
use lsp_server::Connection;
use lsp_types::InitializeResult;
use lsp_types::ServerInfo;
use simplelog::ColorChoice;
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::SharedLogger;
use simplelog::TermLogger;
use simplelog::TerminalMode;
use vsp_error::VspResult;
use vsp_support::json::from_json;

use crate::server::server_capabilities;

/// Run language server.
pub fn run_server() -> VspResult<()> {
  // Initialize.
  init_logger();

  trace!("Ready to run server: {}", env!("CARGO_PKG_VERSION"));

  let (connection, io_threads) = Connection::stdio();
  let (initialize_id, initialize_params) = connection.initialize_start().unwrap();
  info!("Initializing parameters: {}", initialize_params);

  let initialize_params =
    from_json::<lsp_types::InitializeParams>("InitializeParams", &initialize_params).unwrap();

  let initialize_result = InitializeResult {
    capabilities: server_capabilities(),
    server_info:  Some(ServerInfo {
      name:    String::from(""),
      version: Some(String::from(env!("CARGO_PKG_VERSION"))),
    }),
  };
  let initialize_result = serde_json::to_value(initialize_result).unwrap();

  io_threads.join().unwrap();
  Ok(())
}

pub fn init_logger() {
  let loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )];
  CombinedLogger::init(loggers).unwrap();
}
