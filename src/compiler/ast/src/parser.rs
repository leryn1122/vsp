use std::collections::HashMap;

use vsp_token::TokenStream;

use crate::ast::function::Function;
use crate::ast::function::FunctionAccessibility;
use crate::ast::function::FunctionSignature;
use crate::ast::module::Module;
use crate::ast::module::Package;
use crate::ast::types::Parameter;
use crate::ast::types::Type;
use crate::visitor::CompilationUnitASTVisitor;
use crate::visitor::FunctionSignatureASTVisitor;

/// Parser is a syntax analysis automaton that consumes the token stream from
/// the lexer.
pub struct Parser {}

impl Parser {
  pub fn new() -> Self {
    Self {}
  }

  pub fn receive(&self, tokens: &TokenStream) {}
}

impl CompilationUnitASTVisitor for Parser {
  fn visit_shebang(&mut self, shebang: &Option<String>) {
    todo!()
  }

  fn visit_modules(&mut self, modules: &HashMap<String, Module>) {
    todo!()
  }

  fn visit_functions(&mut self, functions: &HashMap<String, Function>) {
    todo!()
  }
}

impl FunctionSignatureASTVisitor for Parser {
  fn visit_accessibility(&mut self, accessibility: &FunctionAccessibility) {
    todo!()
  }

  fn visit_parameters(&mut self, parameters: &Vec<Parameter>) {
    todo!()
  }

  fn visit_return_type(&mut self, return_type: &Type) {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use vsp_token::TokenStream;

  use super::*;
  use crate::ast::CompilationUnit;
}
