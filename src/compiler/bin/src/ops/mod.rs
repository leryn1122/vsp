use vsp_error::VspResult;

pub(crate) mod clean;
pub(crate) mod compile;
pub(crate) mod completion;
pub(crate) mod debug;
pub(crate) mod dump;
pub(crate) mod lsp;
pub(crate) mod new;
pub(crate) mod pm;
pub(crate) mod repl;
pub(crate) mod test;

#[doc(hidden)]
pub trait Entrypoint {
  /// Entrypoint for CLI.
  /// Implementations should include early processing the arguments and passing them to the related
  /// module entrypoint functions.
  /// Early processing is required to adapt the parameters list of entrypoints respective.
  fn entrypoint(&mut self) -> VspResult<()>;
}
