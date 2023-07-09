use vsp_support::ptr::SharedPtr;

use crate::node::NodeId;

/// <h1>Expression</h1>
#[derive(Debug)]
pub struct Expression {
  id: NodeId,
  kind: ExpressionKind,
}

/// <h1>Expression Kind</h1>
#[derive(Debug)]
pub enum ExpressionKind {
  //============================================================================//
  // Literal
  //============================================================================//
  LiteralInteger(i64),
  LiteralFloat(f64),
  LiteralBoolean(bool),
  LiteralString(String),
  Identifier(String),

  //============================================================================//
  // Operations
  //============================================================================//
  /// Unary expression, such as `!foo`.
  Unary(UnaryOpKind, SharedPtr<Expression>),
  /// Binary expression, such as `foo + bar`.
  Binary(
    BinaryOpKind,
    SharedPtr<ExpressionKind>,
    SharedPtr<ExpressionKind>,
  ),

  //============================================================================//
  // Call
  //============================================================================//
  MethodCall(String, SharedPtr<Expression>),
  LambdaExpression(Vec<String>, SharedPtr<Expression>),
}

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
  pub fn as_str(&self) -> &'static str {
    match self {
      UnaryOpKind::Dereference => "*",
      UnaryOpKind::Not => "!",
      UnaryOpKind::Negative => "-",
    }
  }
}

///
#[rustfmt::skip]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BinaryOpKind {
  /*  `+`  */Add,
  /*  `-`  */Subtract,
  /*  `*`  */Multiply,
  /*  `/`  */Division,
  /*  `=`  */Assignment,
}

impl BinaryOpKind {
  pub fn as_str(&self) -> &'static str {
    match *self {
      BinaryOpKind::Add => "+",
      BinaryOpKind::Subtract => "-",
      BinaryOpKind::Multiply => "+",
      BinaryOpKind::Division => "/",
      BinaryOpKind::Assignment => "=",
    }
  }
}
