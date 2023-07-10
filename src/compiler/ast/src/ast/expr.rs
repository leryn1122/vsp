use vsp_support::ptr::SharedPtr;

/// # Expression
///
/// Expression indicates ...
#[derive(Debug)]
pub enum Expression {
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
  /// Unary operation expression, such as `!foo`.
  Unary(UnaryOp, SharedPtr<Expression>),
  /// Binary operation expression, such as `foo + bar`.
  Binary(BinaryOp, SharedPtr<Expression>, SharedPtr<Expression>),

  //============================================================================//
  // Call
  //============================================================================//
  MethodCall(String, SharedPtr<Expression>),
  LambdaExpression(Vec<String>, SharedPtr<Expression>),
}

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

///
#[rustfmt::skip]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BinaryOp {
  /*  `+`  */Add,
  /*  `-`  */Subtract,
  /*  `*`  */Multiply,
  /*  `/`  */Division,
  /*  `=`  */Assignment,
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
