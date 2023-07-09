use crate::ast::expr::ExpressionKind;
use vsp_span::span::Span;

///
pub trait Statement {
  /// True if the statement is an expression.
  fn is_expression(&self) -> bool;
}

/// # Statement
#[derive(Debug)]
pub enum StatementKind {
  /// No operations: Just a single `;`
  NoOp,
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
  Return(Option<ExpressionKind>),
  /// Statement block consisting of statements
  StatementBlock(Box<StatementBlock>),
}

pub struct NoOpStatement;

impl NoOpStatement {}

impl Statement for NoOpStatement {
  #[inline(always)]
  fn is_expression(&self) -> bool {
    false
  }
}

/// Statement represents a if / else statement.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct WhileStatement {}

impl Statement for WhileStatement {
  #[inline(always)]
  fn is_expression(&self) -> bool {
    false
  }
}

/// Statement block contains list of statements.
#[derive(Debug)]
pub struct StatementBlock {
  stmts: Vec<StatementKind>,
}

impl StatementBlock {
  pub fn new() -> Self {
    Self { stmts: vec![] }
  }

  pub fn add_stmt(&mut self, stmt: StatementKind) -> &Self {
    self.stmts.push(stmt);
    self
  }

  pub fn add_stmts<V>(&mut self, stmts: V) -> &Self
  where
    V: IntoIterator<Item = StatementKind>,
  {
    stmts.into_iter().for_each(|s| self.stmts.push(s));
    self
  }
}

impl Default for StatementBlock {
  fn default() -> Self {
    Self::new()
  }
}

impl Statement for StatementBlock {
  fn is_expression(&self) -> bool {
    todo!()
  }
}
