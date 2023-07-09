use crate::parser::token::LocatableToken;
use vsp_ast::ast::expr::ExpressionKind;
use vsp_ast::ast::stmt::{StatementBlock, StatementKind};
use vsp_support::debug_println;

pub struct ParserState {
  tokens: Vec<LocatableToken>,
  exprs: Vec<ExpressionKind>,
  stmts: Vec<StatementKind>,
  stmt_blocks: Vec<StatementBlock>,
  expected_type: Vec<TokenType>,
}

impl ParserState {
  pub fn new() -> Self {
    Self {
      tokens: vec![],
      exprs: vec![],
      stmts: vec![],
      stmt_blocks: vec![],
      expected_type: vec![],
    }
  }

  #[inline]
  pub fn tokens(&self) -> &Vec<LocatableToken> {
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
  pub fn exprs(&self) -> &Vec<ExpressionKind> {
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
  pub fn stmts(&self) -> &Vec<StatementKind> {
    &self.stmts
  }

  #[inline]
  pub fn add_stmt(&mut self, stmt: StatementKind) {
    self.stmts.push(stmt);
  }

  #[inline]
  pub fn stmt_blocks(&self) -> &Vec<StatementBlock> {
    &self.stmt_blocks
  }

  #[inline]
  pub fn add_stmt_blocks(&mut self, stmt_block: StatementBlock) {
    self.stmt_blocks.push(stmt_block)
  }

  #[inline]
  pub fn pop_stmt_blocks(&mut self) -> Option<StatementBlock> {
    self.stmt_blocks.pop()
  }

  #[inline]
  pub(crate) fn add_expected_type(&mut self, expect: TokenType) {
    self.expected_type.push(expect)
  }
}

impl ParserState {
  #[cfg(debug_assertions)]
  pub fn print_tokens(&self) {
    // self.tokens.iter().for_each(|v| debug_println!("{:?}", v));
    // println!();
    debug_println!("State tokens = {:?}", self.tokens);
  }

  #[cfg(debug_assertions)]
  pub fn print_exprs(&self) {
    // self.exprs.iter().for_each(|v| debug_println!("{:?}", v));
    // println!();
    debug_println!("State expressions = {:?}", self.exprs);
  }

  #[cfg(debug_assertions)]
  pub fn print_stmts(&self) {
    // self.stmts.iter().for_each(|v| debug_println!("{:?}", v));
    // println!();
    debug_println!("State statements = {:?}", self.stmts);
  }

  #[cfg(debug_assertions)]
  pub fn print(&self) {
    debug_println!("State tokens = {:?}", self.tokens);
    debug_println!("State expressions = {:?}", self.exprs);
    debug_println!("State statements = {:?}", self.stmts);
    println!();
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
