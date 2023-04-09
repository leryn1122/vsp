use std::collections::HashMap;

use crate::ast::function::Function;
use crate::ast::module::Module;
use crate::ast::CompilationUnit;
use crate::visitor::CompilationUnitASTVisitor;

pub struct ASTPrinter {
  context: ASTPrintContext,
}

impl ASTPrinter {
  pub fn new() -> Self {
    Self {
      context: ASTPrintContext::new(),
    }
  }
}

impl CompilationUnitASTVisitor for ASTPrinter {
  type Value = ();

  fn visit_shebang(&mut self, shebang: &Option<String>) -> Self::Value {
    todo!()
  }

  fn visit_modules(&mut self, module: &HashMap<String, Module>) -> Self::Value {
    todo!()
  }

  fn visit_functions(&mut self, function: &HashMap<String, Function>) -> Self::Value {
    todo!()
  }
}

pub struct ASTPrintContext {
  /** Number of indent level, 1 indent for 2 space. */
  indent: usize,
}

impl ASTPrintContext {
  pub fn new() -> Self {
    Self { indent: 0 }
  }

  pub fn increment(&mut self) {
    self.indent += 1;
  }

  pub fn decrement(&mut self) {
    self.indent -= 1;
  }
}
