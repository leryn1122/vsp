use log::LevelFilter;
use lsp_server::Connection;
use lsp_types::InitializeParams;
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
use crate::server::connection_by_type;
use crate::server::initialize_result;

pub mod config;
pub(crate) mod server;

pub fn init_logger() {
  let loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
    LevelFilter::Trace,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )];
  CombinedLogger::init(loggers).unwrap();
}

/// Run language server.
pub fn start_server(config: Configuration) -> VspResult<()> {
  // Initialize.
  init_logger();

  let (connection, io_threads) = match connection_by_type(&config) {
    Ok(res) => res,
    Err(e) => return Err(VspError::from(e)),
  };
  accept_initialize_request(&connection)
    .and_then(|_| main_loop(&connection))
    .and_then(|_| io_threads.join().map_err(VspError::from))
}

pub fn accept_initialize_request(connection: &Connection) -> VspResult<()> {
  let (initialize_id, initialize_params) = connection.initialize_start().unwrap();
  log::trace!("InitializeParams: {}", initialize_params);

  let initialize_params = from_json::<InitializeParams>("InitializeParams", &initialize_params);
  if initialize_params.is_err() {
    return Err(VspError::from(initialize_params.unwrap_err()));
  }

  let result = connection.initialize_finish(
    initialize_id,
    serde_json::to_value(initialize_result()).unwrap(),
  );
  if result.is_err() {
    return Err(VspError::from(result.unwrap_err()));
  }
  Ok(())
}

pub fn main_loop(connection: &Connection) -> VspResult<()> {
  // TODO: Implement your LSP here.
  Ok(())
}
