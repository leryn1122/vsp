//! It is designed to access and traverse the AST in the visitor pattern.
//!
//! Each AST node type requires at least three traits/structs.
//! - AST visitable for AST nodes.
//! - AST visitable implementations for AST nodes.
//! - AST visitor which call the
//!
//! For example,
use std::collections::HashMap;

use crate::ast::annotation::Annotation;
use crate::ast::CompilationUnit;
use crate::ast::function::Function;
use crate::ast::function::FunctionAccessibility;
use crate::ast::function::FunctionSignature;
use crate::ast::module::Module;
use crate::ast::types::Parameter;
use crate::ast::types::Type;

pub trait CompilationUnitASTVisitor {
  #[allow(unused_variables)]
  fn before_visit(&mut self, compilation_unit: &CompilationUnit) {}
  #[allow(unused_variables)]
  fn after_visit(&mut self, compilation_unit: &CompilationUnit) {}
  fn visit_shebang(&mut self, shebang: &Option<String>);
  fn visit_modules(&mut self, modules: &HashMap<String, Module>);
  fn visit_functions(&mut self, functions: &HashMap<String, Function>);
}

pub trait CompilationUnitASTVisitable {
  fn accept<V: CompilationUnitASTVisitor>(&mut self, visitor: &mut V);
}

impl CompilationUnitASTVisitable for CompilationUnit {
  fn accept<V: CompilationUnitASTVisitor>(&mut self, visitor: &mut V) {
    visitor.before_visit(self);
    visitor.visit_shebang(&self.shebang);
    visitor.visit_modules(&self.modules);
    visitor.visit_functions(&self.functions);
    visitor.after_visit(self);
  }
}

pub trait FunctionASTVisitor {
  #[allow(unused_variables)]
  fn before_visit(&mut self, function: &Function) {}
  #[allow(unused_variables)]
  fn after_visit(&mut self, function: &Function) {}
  fn visit_name(&mut self, name: &str);
  fn visit_annotations(&mut self, annotation: &Option<Vec<Annotation>>);
  fn visit_signature(&mut self, signature: &FunctionSignature);
}

pub trait FunctionASTVisitable {
  fn accept<V: FunctionASTVisitor>(&mut self, visitor: &mut V);
}

impl FunctionASTVisitable for Function {
  fn accept<V: FunctionASTVisitor>(&mut self, visitor: &mut V) {
    visitor.before_visit(self);
    visitor.visit_name(&self.name);
    visitor.visit_annotations(&self.annotations);
    visitor.visit_signature(&self.signature);
    visitor.after_visit(self);
  }
}

pub trait FunctionSignatureASTVisitor {
  #[allow(unused_variables)]
  fn before_visit(&mut self, function_signature: &FunctionSignature) {}
  #[allow(unused_variables)]
  fn after_visit(&mut self, function_signature: &FunctionSignature) {}
  fn visit_accessibility(&mut self, accessibility: &FunctionAccessibility);
  #[allow(clippy::ptr_arg)]
  fn visit_parameters(&mut self, parameters: &Vec<Parameter>);
  fn visit_return_type(&mut self, return_type: &Type);
}

pub trait FunctionSignatureASTVisitable {
  fn accept<V: FunctionSignatureASTVisitor>(&mut self, visitor: &mut V);
}

impl FunctionSignatureASTVisitable for FunctionSignature {
  fn accept<V: FunctionSignatureASTVisitor>(&mut self, visitor: &mut V) {
    visitor.before_visit(self);
    visitor.visit_accessibility(&self.accessibility);
    visitor.visit_parameters(&self.parameters);
    visitor.visit_return_type(&self.return_type);
    visitor.after_visit(self);
  }
}
