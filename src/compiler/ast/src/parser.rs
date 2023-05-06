use std::collections::HashMap;

use vsp_token::TokenStream;

use crate::ast::function::Function;
use crate::ast::function::FunctionAccessibility;
use crate::ast::module::Module;
use crate::ast::types::Parameter;
use crate::ast::types::Type;
use crate::visitor::CompilationUnitASTVisitor;
use crate::visitor::FunctionSignatureASTVisitor;

/// Parser is a syntax analysis automaton that consumes the token stream from
/// the lexer.
#[derive(Default)]
pub struct Parser {}

impl Parser {
  pub fn receive(&self, _tokens: &TokenStream) {}
}

impl CompilationUnitASTVisitor for Parser {
  fn visit_shebang(&mut self, _shebang: &Option<String>) {
    todo!()
  }

  fn visit_modules(&mut self, _modules: &HashMap<String, Module>) {
    todo!()
  }

  fn visit_functions(&mut self, _functions: &HashMap<String, Function>) {
    todo!()
  }
}

impl FunctionSignatureASTVisitor for Parser {
  fn visit_accessibility(&mut self, _accessibility: &FunctionAccessibility) {
    todo!()
  }

  fn visit_parameters(&mut self, _parameters: &Vec<Parameter>) {
    todo!()
  }

  fn visit_return_type(&mut self, _return_type: &Type) {
    todo!()
  }
}
