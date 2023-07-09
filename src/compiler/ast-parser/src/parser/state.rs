use crate::token::LocatableToken;
use vsp_ast::ast::expr::ExpressionKind;
use vsp_ast::ast::stmt::StatementKind;

pub struct ParserState {
  tokens: Vec<LocatableToken>,
  exprs: Vec<ExpressionKind>,
  stmts: Vec<StatementKind>,
  expectedType: Vec<TokenType>,
}

impl ParserState {
  pub fn new() -> Self {
    Self {
      tokens: vec![],
      exprs: vec![],
      stmts: vec![],
      expectedType: vec![],
    }
  }

  #[inline]
  pub fn tokens(&mut self) -> &Vec<LocatableToken> {
    &self.tokens
  }

  #[inline]
  pub fn add_token(&mut self, token: LocatableToken) {
    self.tokens.push(token);
  }

  #[inline]
  pub fn pop_token(&mut self) -> Option<LocatableToken> {
    self.tokens.pop()
  }

  #[inline]
  pub fn exprs(&mut self) -> &Vec<ExpressionKind> {
    &self.exprs
  }

  #[inline]
  pub fn add_expr(&mut self, expr: ExpressionKind) {
    self.exprs.push(expr);
  }

  #[inline]
  pub fn pop_expr(&mut self) -> Option<ExpressionKind> {
    self.exprs.pop()
  }

  #[inline]
  pub fn stmts(&mut self) -> &Vec<StatementKind> {
    &self.stmts
  }

  #[inline]
  pub fn add_stmt(&mut self, stmt: StatementKind) {
    self.stmts.push(stmt);
  }

  #[inline]
  pub(crate) fn add_expected_type(&mut self, expect: TokenType) {
    self.expectedType.push(expect)
  }
}

impl Default for ParserState {
  fn default() -> Self {
    Self::new()
  }
}

pub(crate) enum TokenType {
  Op,
  Type,
}
