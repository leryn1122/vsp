use vsp_span::span::Span;

use crate::ast::expr::Expression;
use crate::node::NodeId;

///
pub trait Statement {
  /// True if the statement is an expression.
  fn is_expression(&self) -> bool;
}

/// # Statement
enum StatementKind {
  /// No operations: Just a single `;`
  NoOp(NoOpStatement),
  // Expression(SharedPtr<Expression>),
  // Assignment(String, Expression),

  //================================================================//
  // Procedure control flow
  //================================================================//
  If(IfStatement),
  While(WhileStatement),

  // Declarations
  // VariableDeclaration(String, Option<Expression>),
  // FunctionDeclaration(String, Option<Expression>),

  // Blocks
  // Return(Option<Expression>),
  /// Statement block consisting of statements
  StatementBlock(Box<StatementBlock>),
}

pub struct NoOpStatement;

impl Statement for NoOpStatement {
  #[inline(always)]
  fn is_expression(&self) -> bool {
    false
  }
}

/// Statement represents a if / else statement.
pub struct IfStatement {
  pub span: Span,
}

impl Statement for IfStatement {
  #[inline(always)]
  fn is_expression(&self) -> bool {
    true
  }
}

/// Statement represents a while statement.
pub struct WhileStatement {}

impl Statement for WhileStatement {
  #[inline(always)]
  fn is_expression(&self) -> bool {
    false
  }
}

pub struct StatementBlock {}

impl Statement for StatementBlock {
  fn is_expression(&self) -> bool {
    todo!()
  }
}
