use std::collections::HashMap;

use vsp_span::span::Span;

use crate::ast::annotation::Annotation;
use crate::ast::CompilationUnit;
use crate::ast::function::Function;
use crate::ast::function::FunctionSignature;
use crate::ast::module::Module;
use crate::visitor::CompilationUnitASTVisitor;
use crate::visitor::FunctionASTVisitor;

pub struct ASTPrinter {
  context: ASTPrintContext,
}

impl ASTPrinter {
  pub fn new() -> Self {
    Self {
      context: ASTPrintContext::new(),
    }
  }

  fn indent(&mut self) {
    self.context.increment()
  }

  fn unindent(&mut self) {
    self.context.decrement()
  }

  fn print_location(&self, span: &Span) -> String {
    let arr = span.expand_as_array();
    format!("start:{}:{}, end:{}:{}", arr[0], arr[1], arr[2], arr[3])
  }

  fn print_segment(&self, types: &str, span: &Span) -> String {
    format!("`-{} <{}>", types, self.print_location(span))
  }
}

impl Default for ASTPrinter {
  fn default() -> Self {
    Self::new()
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

impl Default for ASTPrintContext {
  fn default() -> Self {
    Self::new()
  }
}

impl CompilationUnitASTVisitor for ASTPrinter {
  fn before_visit(&mut self, compilation_unit: &CompilationUnit) {
    println!(
      "CompilationUnit <file:`{}`>",
      compilation_unit.meta.filename
    );
  }

  fn visit_shebang(&mut self, _shebang: &Option<String>) {
    println!(
      "{}",
      self.print_segment("ShebangDeclaration", &Span::default())
    )
  }

  fn visit_modules(&mut self, modules: &HashMap<String, Module>) {
    for module in modules {
      println!("ModuleDeclaration: {:?}", module.0)
    }
  }

  fn visit_functions(&mut self, functions: &HashMap<String, Function>) {
    for function in functions {
      println!("FunctionDeclaration: {:?}", function.0)
    }
  }
}

impl FunctionASTVisitor for ASTPrinter {
  fn visit_name(&mut self, name: &str) {
    todo!()
  }

  fn visit_annotations(&mut self, _annotation: &Option<Vec<Annotation>>) {
    todo!()
  }

  fn visit_signature(&mut self, _signature: &FunctionSignature) {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use crate::ast::CompilationUnit;
  use crate::visitor::CompilationUnitASTVisitable;

  use super::*;

  #[test]
  fn test_indent() {
    let mut printer = ASTPrinter::new();
    let mut unit = CompilationUnit::new("hello-world.vsp");
    unit.accept(&mut printer);
  }
}
