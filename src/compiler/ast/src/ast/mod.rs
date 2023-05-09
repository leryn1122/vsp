use std::collections::HashMap;

use vsp_span::span::Span;

use crate::ast::function::Function;
use crate::ast::module::Module;

pub mod annotation;
pub mod expr;
pub mod function;
pub mod modifier;
pub mod module;
pub mod stmt;
pub mod types;

pub type AST = CompilationUnit;

/// A single file is considered as a compilation unit, known as a translation unit in `clang`.
///
/// It is also the root of AST (abstract syntax tree) which all the items in a single source file
/// are mounted at.
pub struct CompilationUnit {
  pub meta:      FsMeta,
  pub span:      Span,
  pub shebang:   Option<String>,
  pub modules:   HashMap<String, Module>,
  pub functions: HashMap<String, Function>,
}

impl CompilationUnit {
  pub fn new(filename: &str) -> Self {
    Self {
      meta:      FsMeta {
        filename: filename.to_string(),
      },
      span:      Span::default(),
      shebang:   None,
      modules:   HashMap::new(),
      functions: HashMap::new(),
    }
  }

  pub fn add_function(&mut self, function: Function) {
    self.functions.insert(function.name.clone(), function);
  }
}

/// FsMeta means filesystem metadata, the essential information about the source
/// files, including filename, etc.
pub struct FsMeta {
  pub(crate) filename: String,
}

#[derive(Clone)]
struct Identifier {}
