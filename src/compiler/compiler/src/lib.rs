use target_lexicon::Triple;

use crate::option::TargetOptions;

pub mod option;

pub fn compile() {
  let compiler = Compiler::instance();

  compiler.create_diagnostics();

  let mut target_options = TargetOptions::default();
  target_options.triplet = Triple::host();
  compiler.set_target_options(target_options);
  compiler.create_file_manager();
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
    Self {
      target_options: target_options,
    }
  }

  pub fn create_diagnostics(&self) -> () {}

  pub fn create_file_manager(&self) -> () {}

  pub fn create_preprocessor(&self) -> () {}

  pub fn create_ast_context(&self) -> () {}
}
