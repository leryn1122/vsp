//! It is designed to access and traverse the AST in the visitor pattern.
//!
//! Each AST node type requires at least three traits/structs.
//! - AST visitable for AST nodes.
//! - AST visitable implementations for AST nodes.
//! - AST visitor which call the
//!
//! For example,
//!
//! ```rust
//! use vsp_ast::ast::annotation::Annotation;
//! use vsp_ast::ast::function::Function;
//! use vsp_ast::ast::function::FunctionSignature;
//!
//! pub trait FunctionASTVisitable {
//!   fn accept<V: FunctionASTVisitor>(&mut self, visitor: &mut V);
//! }
//!
//! impl FunctionASTVisitable for Function {
//!   fn accept<V: FunctionASTVisitor>(&mut self, visitor: &mut V) {
//!     visitor.visit_name(&self.name);
//!     visitor.visit_annotations(&self.annotations);
//!     visitor.visit_signature(&self.signature);
//!   }
//! }
//!
//! pub trait FunctionASTVisitor {
//!   fn before_visit(&mut self);
//!   fn after_visit(&mut self);
//!   fn visit_name(&mut self, name: &String);
//!   fn visit_annotations(&mut self, annotation: &Option<Vec<Annotation>>);
//!   fn visit_signature(&mut self, signature: &FunctionSignature);
//! }
//! ```
use std::collections::HashMap;

use crate::ast::annotation::Annotation;
use crate::ast::function::Function;
use crate::ast::function::FunctionAccessibility;
use crate::ast::function::FunctionSignature;
use crate::ast::module::Module;
use crate::ast::types::Parameter;
use crate::ast::types::Type;
use crate::ast::CompilationUnit;

pub trait CompilationUnitASTVisitor {
  fn before_visit(&mut self) {}
  fn after_visit(&mut self) {}
  fn visit_shebang(&mut self, shebang: &Option<String>);
  fn visit_modules(&mut self, modules: &HashMap<String, Module>);
  fn visit_functions(&mut self, functions: &HashMap<String, Function>);
}

pub trait CompilationUnitASTVisitable {
  fn accept<V: CompilationUnitASTVisitor>(&mut self, visitor: &mut V);
}

impl CompilationUnitASTVisitable for CompilationUnit {
  fn accept<V: CompilationUnitASTVisitor>(&mut self, visitor: &mut V) {
    visitor.before_visit();
    visitor.visit_shebang(&self.shebang);
    visitor.visit_modules(&self.modules);
    visitor.visit_functions(&self.functions);
    visitor.after_visit();
  }
}

pub trait FunctionASTVisitor {
  fn before_visit(&mut self) {}
  fn after_visit(&mut self) {}
  fn visit_name(&mut self, name: &String);
  fn visit_annotations(&mut self, annotation: &Option<Vec<Annotation>>);
  fn visit_signature(&mut self, signature: &FunctionSignature);
}

pub trait FunctionASTVisitable {
  fn accept<V: FunctionASTVisitor>(&mut self, visitor: &mut V);
}

impl FunctionASTVisitable for Function {
  fn accept<V: FunctionASTVisitor>(&mut self, visitor: &mut V) {
    visitor.before_visit();
    visitor.visit_name(&self.name);
    visitor.visit_annotations(&self.annotations);
    visitor.visit_signature(&self.signature);
    visitor.after_visit();
  }
}

pub trait FunctionSignatureASTVisitor {
  fn before_visit(&mut self) {}
  fn after_visit(&mut self) {}
  fn visit_accessibility(&mut self, accessibility: &FunctionAccessibility);
  fn visit_parameters(&mut self, parameters: &Vec<Parameter>);
  fn visit_return_type(&mut self, return_type: &Type);
}

pub trait FunctionSignatureASTVisitable {
  fn accept<V: FunctionSignatureASTVisitor>(&mut self, visitor: &mut V);
}

impl FunctionSignatureASTVisitable for FunctionSignature {
  fn accept<V: FunctionSignatureASTVisitor>(&mut self, visitor: &mut V) {
    visitor.before_visit();
    visitor.visit_accessibility(&self.accessibility);
    visitor.visit_parameters(&self.parameters);
    visitor.visit_return_type(&self.return_type);
    visitor.after_visit();
  }
}
