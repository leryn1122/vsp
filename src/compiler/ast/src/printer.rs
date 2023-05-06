use std::collections::HashMap;

use vsp_span::span::Span;

use crate::ast::annotation::Annotation;
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
    let array = span.expand_as_array();
    format!(
      "start:{}:{}, end:{}:{}",
      array[0], array[1], array[2], array[3]
    )
  }

  ///
  ///
  ///
  /// ```plaintext
  /// ModuleDeclaration <col:12, col:16> col:16 used num 'int'
  /// ```
  fn print_segment(&self, types: &str, span: &Span) -> String {
    format!("`-{} <{}>", types, self.print_location(&span))
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
  fn before_visit(&mut self) {
    println!("CompilationUnit");
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
  fn visit_name(&mut self, _name: &String) {
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
  use super::*;
  use crate::ast;
  use crate::ast::CompilationUnit;
  use crate::ast::FsMeta;
  use crate::visitor::CompilationUnitASTVisitable;

  #[test]
  fn test_indent() {
    let mut printer = ASTPrinter::new();
    let mut root = CompilationUnit::new("hello-world.vsp");
    root.accept(&mut printer);
  }
}
