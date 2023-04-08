use crate::ast::CompilationUnit;

pub struct ASTPrinter {
  root: Box<CompilationUnit>,
}

impl ASTPrinter {
  pub fn new(root: CompilationUnit) -> Self {
    Self {
      root: Box::new(root),
    }
  }

  fn visit_function(&self, name: &str) {}
}
