//! It is designed to access and traverse the AST in the visitor pattern.
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
  type Value;
  fn visit_shebang(&mut self, shebang: &Option<String>) -> Self::Value;
  fn visit_modules(&mut self, module: &HashMap<String, Module>) -> Self::Value;
  fn visit_functions(&mut self, function: &HashMap<String, Function>) -> Self::Value;
}

pub trait CompilationUnitASTVisitable {
  fn accept<V: CompilationUnitASTVisitor>(&mut self, visitor: &mut V);
}

impl CompilationUnitASTVisitable for CompilationUnit {
  fn accept<V: CompilationUnitASTVisitor>(&mut self, visitor: &mut V) {
    visitor.visit_shebang(&self.shebang);
    visitor.visit_modules(&self.modules);
    visitor.visit_functions(&self.functions);
  }
}

pub trait FunctionASTVisitor {
  type Value;
  fn visit_name(&mut self, name: &String) -> Self::Value;
  fn visit_annotations(&mut self, annotation: &Option<Vec<Annotation>>) -> Self::Value;
  fn visit_signature(&mut self, signature: &FunctionSignature) -> Self::Value;
}

pub trait FunctionASTVisitable {
  fn accept<V: FunctionASTVisitor>(&mut self, visitor: &mut V);
}

impl FunctionASTVisitable for Function {
  fn accept<V: FunctionASTVisitor>(&mut self, visitor: &mut V) {
    visitor.visit_name(&self.name);
    visitor.visit_annotations(&self.annotations);
    visitor.visit_signature(&self.signature);
  }
}

pub trait FunctionSignatureASTVisitor {
  type Value;
  fn visit_accessibility(&mut self, accessibility: &FunctionAccessibility) -> Self::Value;
  fn visit_parameters(&mut self, parameters: &Vec<Parameter>) -> Self::Value;
  fn visit_return_type(&mut self, return_type: &Type) -> Self::Value;
}

pub trait FunctionSignatureASTVisitable {
  fn accept<V: FunctionSignatureASTVisitor>(&mut self, visitor: &mut V);
}

impl FunctionSignatureASTVisitable for FunctionSignature {
  fn accept<V: FunctionSignatureASTVisitor>(&mut self, visitor: &mut V) {
    visitor.visit_accessibility(&self.accessibility);
    visitor.visit_parameters(&self.parameters);
    visitor.visit_return_type(&self.return_type);
  }
}
