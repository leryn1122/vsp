use crate::option::TargetOptions;

pub mod option;

/// Entrypoint to compile the source codes.
pub fn compile() {
  let compiler = Compiler::instance();

  compiler.create_diagnostics();

  let mut target_options = TargetOptions::default();
  compiler.set_target_options(target_options);
  // let mut file_manager: FileManager = compiler.create_file_manager();
  // let source_manager = compiler.create_source_manager(&mut file_manager);
  compiler.create_preprocessor();
  compiler.create_ast_context();
}

pub struct Compiler {
  target_options: TargetOptions,
}

impl Compiler {
  /// Create a new compiler instance with all default fields.
  pub fn instance() -> Self {
    Self {
      target_options: TargetOptions::default(),
    }
  }

  pub fn set_target_options(&self, target_options: TargetOptions) -> Self {
    Self { target_options }
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

  pub fn create_preprocessor(&self) -> () {}

  pub fn create_ast_context(&self) -> () {}
}
