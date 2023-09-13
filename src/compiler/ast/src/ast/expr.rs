use crate::ast::ASTNode;
use crate::ast::ExprNode;

/// # Expression
///
/// Expression indicates ...
#[derive(Clone, Debug)]
pub enum Expression {
  Unit,

  // Literal
  LiteralInteger(i64),
  LiteralFloat(f64),
  LiteralBoolean(bool),
  LiteralString(String),
  Identifier(String),

  // Operations
  /// Unary operation expression, such as `!foo`.
  Unary(UnaryOp, Box<Expression>),
  /// Binary operation expression, such as `foo + bar`.
  Binary(BinaryOp, Box<Expression>, Box<Expression>),

  // Call
  MethodCall(String, Box<Expression>),
  LambdaExpression(Vec<String>, Box<Expression>),
}

impl ASTNode for Expression {}

impl ExprNode for Expression {}

/// <h1>Unary Operation</h1>
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UnaryOp {
  /// `*` for dereference the pointer.
  Dereference,
  /// `!` for logical `not`.
  Not,
  /// `-` for numeric negation.
  Negative,
}

impl UnaryOp {
  pub fn as_str(&self) -> &'static str {
    match self {
      UnaryOp::Dereference => "*",
      UnaryOp::Not => "!",
      UnaryOp::Negative => "-",
    }
  }
}

/// Binary operation refers to the operation which takes two parameters.
#[rustfmt::skip]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BinaryOp {
  /// `+` for addition.
  Add,
  /// `-` for subtraction.
  Subtract,
  /// `*` for multiplication.
  Multiply,
  /// `/` for division.
  Division,
  /// `=` for assignment.
  Assignment,
}

impl BinaryOp {
  pub fn as_str(&self) -> &'static str {
    match *self {
      BinaryOp::Add => "+",
      BinaryOp::Subtract => "-",
      BinaryOp::Multiply => "+",
      BinaryOp::Division => "/",
      BinaryOp::Assignment => "=",
    }
  }
}
