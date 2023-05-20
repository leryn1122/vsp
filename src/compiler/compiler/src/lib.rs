use crate::dispatch::CompilationDispatcher;
use crate::option::TargetOptions;

pub mod dispatch;
pub mod option;

/// Entrypoint to compile the source codes.
pub fn compile() {
  let target_options = TargetOptions::default();
  let dispatcher = CompilationDispatcher::default();
  let compiler = CompilerInstance::from(dispatcher, target_options);
  compiler.create_diagnostics();
  // let mut file_manager: FileManager = compiler.create_file_manager();
  // let source_manager = compiler.create_source_manager(&mut file_manager);
  compiler.create_preprocessor();
  compiler.create_ast_context();
}

/// Compiler instance for all the compilation job.
pub struct CompilerInstance {
  diagnostics:    Option<()>,
  dispatcher:     CompilationDispatcher,
  preprocessor:   Option<()>,
  source_manager: (),
  target_options: TargetOptions,
}

impl CompilerInstance {
  pub fn from(dispatcher: CompilationDispatcher, target_options: TargetOptions) -> Self {
    Self {
      diagnostics: None,
      dispatcher,
      preprocessor: None,
      source_manager: (),
      target_options,
    }
  }

  pub fn set_target_options(&mut self, target_options: TargetOptions) {
    self.target_options = target_options;
  }

  pub fn create_diagnostics(&self) {}

  // pub fn create_file_manager<F>(&self) -> F
  // where
  //   F: FileManager + Sized,
  // {
  //   DefaultFileManager {}
  // }
  //
  // pub fn create_source_manager<F>(&self, file_manager: &mut F) -> ()
  // where
  //   F: FileManager + Sized,
  // {
  // }

  pub fn create_preprocessor(&self) {}

  pub fn create_ast_context(&self) {}
}
