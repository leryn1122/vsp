use std::time::Instant;

use log::LevelFilter;
use lsp_server::Connection;
use lsp_server::Message;
use lsp_types::InitializeParams;
use simplelog::ColorChoice;
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::SharedLogger;
use simplelog::TermLogger;
use simplelog::TerminalMode;
use threadpool::ThreadPool;
use vsp_error::VspError;
use vsp_error::VspResult;

use crate::config::Configuration;
use crate::server::connection_by_type;
use crate::server::server_capabilities_as_value;

pub mod config;
pub mod server;

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
pub fn start_lsp_server(config: Configuration) -> VspResult<()> {
  init_logger();

  let (connection, io_threads) = connection_by_type(&config).map_err(VspError::from)?;

  let initialize_params = connection.initialize(server_capabilities_as_value()).unwrap();
  // TODO: handle error here
  main_loop(connection, initialize_params).unwrap();

  let res = io_threads.join().map_err(VspError::from);
  log::warn!("Shutting down the language server");
  res
}

pub fn main_loop(connection: Connection, initialize_params: serde_json::Value) -> VspResult<()> {
  // TODO: Implement your LSP here.

  let workers = std::thread::available_parallelism().unwrap().get();
  let pool = ThreadPool::new(workers);
  let initialize_params: InitializeParams = serde_json::from_value(initialize_params).unwrap();

  log::info!("Starting main loop");

  for message in &connection.receiver {
    let now = Instant::now();
    if let Message::Request(req) = &message {
      if connection.handle_shutdown(req).unwrap() {
        return Ok(());
      }
    }
  }

  Ok(())
}
