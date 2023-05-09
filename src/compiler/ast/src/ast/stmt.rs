use vsp_support::ptr::SharedPtr;

use crate::ast::expr::Expression;
use crate::node::NodeId;

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

/// # Statement
enum StatementKind {
  /// No operations: Just a single `;`
  NoOp,
  Expression(SharedPtr<Expression>),
  Assignment(String, Expression),

  //================================================================//
  // Procedure control flow
  //================================================================//
  If(
    Expression,
    SharedPtr<Statement>,
    Option<SharedPtr<Statement>>,
  ),
  While(Expression, SharedPtr<Statement>),

  // Declarations
  VariableDeclaration(String, Option<Expression>),
  FunctionDeclaration(String, Option<Expression>),

  // Blocks
  Return(Option<Expression>),
  /// Statement block consisting of statements
  StatementBlock(SharedPtr<StatementBlock>),
}

///
pub struct StatementBlock {
  id:         NodeId,
  statements: Vec<Statement>,
}
