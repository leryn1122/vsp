use vsp_error::VspResult;

pub(crate) mod build;
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
pub trait Entrypoint: clap::Args {
  fn entrypoint(&mut self) -> VspResult<()>;
}
