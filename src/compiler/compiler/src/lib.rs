use getset::Getters;
use getset::Setters;
use vsp_diag::DiagnosticEngine;

use crate::action::CompilationInvocation;
use crate::dispatch::CompilationDispatcher;
use crate::option::LangOptions;
use crate::option::TargetOptions;
use crate::source::SourceManager;

pub mod action;
pub mod dispatch;
pub mod option;
pub mod source;

/// Entrypoint to compile the source codes.
pub fn compile(target_options: TargetOptions) -> anyhow::Result<()> {
  let dispatcher = CompilationDispatcher::default();
  let mut compiler = CompilerInstance::from(dispatcher, target_options);

  compiler.create_preprocessor();
  compiler.create_ast_context();

  #[cfg(debug_assertions)]
  compiler.print_status();

  compiler.do_compile()
}

type ASTContext = ();
type InMemoryModuleCache = ();

/// Compiler instance, serves complete context for compilation and owns all necessary components to
/// run the compiler, for example, the target options, AST context.
#[derive(Getters, Setters)]
pub struct CompilerInstance {
  ast_context:    Option<ASTContext>,
  #[getset(get = "pub", set = "pub")]
  diagnostics:    DiagnosticEngine,
  dispatcher:     CompilationDispatcher,
  invocation:     CompilationInvocation,
  lang_options:   LangOptions,
  preprocessor:   Option<()>,
  source_manager: SourceManager,
  module_cache:   InMemoryModuleCache,
  #[getset(get = "pub", set = "pub")]
  target_options: TargetOptions,
}

impl CompilerInstance {
  pub fn from(dispatcher: CompilationDispatcher, target_options: TargetOptions) -> Self {
    Self {
      ast_context: None,
      diagnostics: DiagnosticEngine::default(),
      dispatcher,
      invocation: CompilationInvocation {},
      lang_options: LangOptions::default(),
      module_cache: (),
      preprocessor: None,
      source_manager: SourceManager::default(),
      target_options,
    }
  }

  pub fn create_preprocessor(&mut self) {
    self.preprocessor = Some(())
  }

  pub fn create_ast_context(&mut self) {
    self.ast_context = Some(())
  }
}

impl CompilerInstance {
  #[cfg(debug_assertions)]
  pub fn print_status(&self) {
    self.target_options.print_status();
  }

  pub fn do_compile(&mut self) -> anyhow::Result<()> {
    Ok(())
  }
}
