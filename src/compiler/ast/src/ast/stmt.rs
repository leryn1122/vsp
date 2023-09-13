use vsp_span::Span;

use crate::ast::expr::Expression;
use crate::ast::ASTNode;
use crate::ast::StmtNode;

/// # Statement
#[derive(Clone, Debug)]
pub enum Statement {
  /// No operations: Just a single `;`
  NoOp,
  // Expression(Box<Expression>),
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
  Return(Option<Expression>),
  /// Statement block consisting of statements
  Block(Box<StatementBlock>),
}

impl ASTNode for Statement {}

impl StmtNode for Statement {}

pub struct NoOpStatement;

impl NoOpStatement {}

/// Statement represents a if / else statement.
#[derive(Clone, Debug)]
pub struct IfStatement {
  pub span: Span,
}

/// Statement represents a while statement.
#[derive(Clone, Debug)]
pub struct WhileStatement {}

/// Statement block contains list of statements.
#[derive(Clone, Debug)]
pub struct StatementBlock {
  stmts: Vec<Statement>,
}

impl StatementBlock {
  pub fn new() -> Self {
    Self { stmts: vec![] }
  }

  pub fn from(stmts: Vec<Statement>) -> Self {
    Self { stmts }
  }

  pub fn add_stmt(&mut self, stmt: Statement) -> &Self {
    self.stmts.push(stmt);
    self
  }

  pub fn add_stmts<V>(&mut self, stmts: V) -> &Self
  where
    V: IntoIterator<Item = Statement>,
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
