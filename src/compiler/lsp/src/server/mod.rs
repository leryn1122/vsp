use lsp_server::Connection;
use lsp_server::IoThreads;
use lsp_types::InitializeResult;
use lsp_types::OneOf;
use lsp_types::ServerCapabilities;
use lsp_types::ServerInfo;

use crate::config::Configuration;

/// Create the transport depending on the configuration type.
pub(crate) fn connection_by_type(
  config: &Configuration,
) -> std::io::Result<(Connection, IoThreads)> {
  if let Some(addr) = config.address {
    log::info!("Start to run server at {}", addr);
    Connection::listen(addr)
  } else if let Some(addr) = &config.socket {
    log::info!("Start to run server at {}", addr);
    Connection::listen(addr.as_str())
  } else {
    log::info!("Start to run server at stdio of pid {}", std::process::id());
    Ok(Connection::stdio())
  }
}

pub fn initialize_result() -> InitializeResult {
  InitializeResult {
    capabilities: server_capabilities(),
    server_info:  Some(server_info()),
  }
}

pub fn server_capabilities() -> ServerCapabilities {
  ServerCapabilities {
    definition_provider: Some(OneOf::Left(true)),
    ..Default::default()
  }
}

pub fn server_capabilities_as_value() -> serde_json::Value {
  // Unnecessary to check.
  serde_json::to_value(server_capabilities()).unwrap()
}

pub fn server_info() -> ServerInfo {
  ServerInfo {
    name:    String::from("vsp-lsp-server"),
    version: Some(String::from(env!("CARGO_PKG_VERSION"))),
  }
}
