use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use getset::Getters;
use getset::Setters;

use vsp_diag::DiagnosticEngine;
use vsp_error::{VspError, VspResult};
use vsp_fs::manager::VFSManager;
use vsp_fs::path::VFSPath;

use crate::dispatch::CompilationDispatcher;
use crate::option::LangOptions;
use crate::option::TargetOptions;
use crate::source::SourceManager;

pub mod action;
pub mod db;
pub mod dispatch;
pub mod option;
pub mod source;

/// Entrypoint to compile the source codes.
pub fn start_compile(filename: &PathBuf, target_options: TargetOptions) -> VspResult<()> {
  let dispatcher = CompilationDispatcher::default();
  let mut compiler = CompilerInstance::from(dispatcher, target_options);

  compiler.create_preprocessor();
  compiler.create_ast_context();

  #[cfg(debug_assertions)]
  compiler.debug_print_status();

  compiler.run(filename)
}

type ASTContext = ();
type InMemoryModuleCache = ();

/// Compiler instance, serves complete context for compilation and owns all necessary components to
/// run the compiler, for example, the target options, AST context.
#[derive(Getters, Setters)]
pub struct CompilerInstance {
  /// AST context.
  ast_context: Option<ASTContext>,
  #[getset(get = "pub", set = "pub")]
  diagnostics: DiagnosticEngine,
  dispatcher: CompilationDispatcher,
  lang_options: LangOptions,
  preprocessor: Option<()>,
  vfs_manager: VFSManager,
  #[getset(get = "pub", set = "pub")]
  source_manager: SourceManager,
  module_cache: InMemoryModuleCache,
  #[getset(get = "pub", set = "pub")]
  target_options: TargetOptions,
}

impl CompilerInstance {
  pub fn from(dispatcher: CompilationDispatcher, target_options: TargetOptions) -> Self {
    let vfs_manager = VFSManager::default();
    Self {
      ast_context: None,
      diagnostics: DiagnosticEngine::default(),
      dispatcher,
      lang_options: LangOptions::default(),
      module_cache: (),
      preprocessor: None,
      vfs_manager,
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
  /// Print the initialized status of the compiler in debug mode.
  #[cfg(debug_assertions)]
  pub fn debug_print_status(&self) {
    self.target_options.debug_print_status();
  }

  pub fn run(&mut self, file: &PathBuf) -> VspResult<()> {
    let path = VFSPath::from_str(file.to_str().unwrap());
    let mut file = self.vfs_manager.get_file(&path).unwrap();
    let main_file = self.source_manager.create_main_file_id(&file);

    let mut buf = String::new();
    let _ = file.as_mut().read_to_string(&mut buf).unwrap();

    Ok(())
  }
}

pub struct SimpleCompilerInstance;

impl SimpleCompilerInstance {
  pub fn new() -> Self {
    Self {}
  }

  pub fn compile(&mut self, path: &PathBuf) -> VspResult<()> {
    let mut file = File::open(path).map_err(|e| VspError::from(e))?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let mut lex = vsp_ast_parser::lex::DefaultLexer {};
    let mut tokens = lex.tokenize(buf.as_str()).unwrap();
    let mut parser = vsp_ast_parser::parser::DefaultParser {};
    let _ = parser.parse(tokens);
    Ok(())
  }
}
