pub mod annotation;
pub mod module;

use crate::node::NodeId;

use vsp_support::ptr::SharedPtr;

/// <h1>Expression</h1>
pub struct Expression {
  id: NodeId,
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
  id: NodeId,
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

/// <h1>Function</h1>
///
pub struct Function {
  signature: FunctionSignature,
  body: Option<SharedPtr<StatementBlock>>,
}

/// Function signature or function declarator.
pub struct FunctionSignature {
  header: FunctionHeader,
  parameters: Vec<Parameter>,
  return_value: Type,
}

pub struct FunctionHeader {
  /// `public` or `private`
  access: FunctionAccessibility,
  /// True if `const`, false by default.
  constness: bool,
}

impl Default for FunctionHeader {
  fn default() -> Self {
    Self {
      access: FunctionAccessibility::Private,
      constness: false,
    }
  }
}

pub enum FunctionAccessibility {
  Public,
  Private,
}

pub struct Parameter;

pub struct Type;

///
pub struct Path;

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
  id: NodeId,
  statements: Vec<Statement>,
}

#[cfg(test)]
mod tests {
  use super::*;
  use vsp_support::ptr::make_shared_ptr;

  #[test]
  pub fn test_construct_helloworld() {
    let function_ptr = StatementKind::FunctionDeclaration(make_shared_ptr(Function {
      signature: FunctionSignature {
        header: Default::default(),
        parameters: vec![],
        return_value: Type {},
      },
      body: None,
    }));
    Statement {
      id: NodeId::from_u32(0),
      kind: StatementKind::Declaration(),
    };
  }
}
