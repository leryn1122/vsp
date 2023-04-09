use std::collections::HashMap;

use crate::ast::function::Function;
use crate::ast::function::FunctionAccessibility;
use crate::ast::function::FunctionSignature;
use crate::ast::module::Module;
use crate::ast::module::Package;
use crate::ast::types::Parameter;
use crate::ast::types::Type;
use crate::visitor::CompilationUnitASTVisitor;
use crate::visitor::FunctionSignatureASTVisitor;

pub struct Parser {}

impl Parser {
  pub fn new() -> Self {
    Self {}
  }
}

impl CompilationUnitASTVisitor for Parser {
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

impl FunctionSignatureASTVisitor for Parser {
  type Value = ();

  fn visit_accessibility(&mut self, accessibility: &FunctionAccessibility) -> Self::Value {
    todo!()
  }

  fn visit_parameters(&mut self, parameters: &Vec<Parameter>) -> Self::Value {
    todo!()
  }

  fn visit_return_type(&mut self, return_type: &Type) -> Self::Value {
    todo!()
  }
}
