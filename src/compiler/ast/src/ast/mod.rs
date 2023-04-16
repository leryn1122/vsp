use std::collections::HashMap;

use vsp_span::span::Span;
use vsp_support::ptr::SharedPtr;

use crate::ast::function::Function;
use crate::ast::module::Module;
use crate::node::NodeId;

pub mod annotation;
pub mod function;
pub mod module;
pub mod types;

pub type AST = CompilationUnit;

/// A single file is considered as a compilation unit.
///
/// It is also the root of AST (abstract syntax tree) which all the items in a
/// single source file are mounted at.
pub struct CompilationUnit {
  pub(crate) meta:      FsMeta,
  pub(crate) span:      Span,
  pub(crate) shebang:   Option<String>,
  pub(crate) modules:   HashMap<String, Module>,
  pub(crate) functions: HashMap<String, Function>,
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

/// Definite of constancy referring to the preserved word `const`.
#[derive(Debug)]
pub enum Constancy {
  Constant,
  None,
}

impl Constancy {
  pub fn is_constant(&self) -> bool {
    match self {
      Constancy::Constant => true,
      Constancy::None => false,
    }
  }
}

/// Accessibility.
/// - `Public`
/// - `Private`
pub enum Accessibility {
  Public,
  Private,
}

/// <h1>Expression</h1>
pub struct Expression {
  id:   NodeId,
  kind: ExpressionKind,
}

impl Expression {
  /// True if the expression returns a valid value.
  pub fn has_return(&self) -> bool {
    /* TODO: True if block statement */
    if true {
      true
    } else {
      false
    }
  }
}

/// <h1>Expression Kind</h1>
enum ExpressionKind {
  /// Unary expression, such as `!foo`.
  Unary(UnaryOpKind, SharedPtr<Expression>),
  /// Binary expression, such as `foo + bar`.
  Binary(BinaryOpKind, SharedPtr<Expression>, SharedPtr<Expression>),
}

/// <h1>Statement</h1>
pub struct Statement {
  id:   NodeId,
  kind: StatementKind,
}

impl Statement {
  ///
  pub fn is_expression(&self) -> bool {
    matches!(self.kind, StatementKind::Expression(_))
  }
}

/// Statement kind.
enum StatementKind {
  /// No operations: Just a single `;`.
  NoOp,
  /// Function declaration.
  FunctionDeclaration(SharedPtr<Function>),
  /// Expression statement.
  Expression(SharedPtr<Expression>),
  /// Statement block consisting of statements.
  StatementBlock(SharedPtr<StatementBlock>),
}

#[derive(Clone)]
struct Identifier {}

/// <h1>Unary Operation</h1>
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UnaryOpKind {
  /// `*` for dereference the pointer.
  Dereference,
  /// `!` for logical `not`.
  Not,
  /// `-` for numeric negation.
  Negative,
}

impl UnaryOpKind {
  pub fn to_string(&self) -> &'static str {
    match self {
      UnaryOpKind::Dereference => "*",
      UnaryOpKind::Not => "!",
      UnaryOpKind::Negative => "-",
    }
  }
}

///
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BinaryOpKind {
  /// `+`
  Add,
  /// `-`
  Subtract,
  /// `*`
  Multiply,
  /// `/`
  Division,
  /// `=`
  Assignment,
}

impl BinaryOpKind {
  pub fn to_string(&self) -> &'static str {
    match *self {
      BinaryOpKind::Add => "+",
      BinaryOpKind::Subtract => "-",
      BinaryOpKind::Multiply => "+",
      BinaryOpKind::Division => "/",
      BinaryOpKind::Assignment => "=",
    }
  }
}

///
pub struct StatementBlock {
  id:         NodeId,
  statements: Vec<Statement>,
}
