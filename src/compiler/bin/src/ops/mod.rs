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

pub trait Entrypoint {
  fn entrypoint(&self) -> anyhow::Result<()>;
}
