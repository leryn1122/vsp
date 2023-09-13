use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use vsp_span::Locatable;
use vsp_span::Span;

use crate::token::Token;

/// The final result of the lexical analysis, which are transferred to the AST parser.
pub type TokenStream = Vec<LocatableToken>;

/// A locatable token with span, as its start and end location in the source codes.
#[derive(Clone)]
pub struct LocatableToken {
  token: Token,
  span:  Span,
}

impl LocatableToken {
  /// New token.
  pub fn new(token: Token, span: Span) -> Self {
    Self { token, span }
  }

  #[inline]
  pub fn token(&self) -> &Token {
    &self.token
  }

  #[inline]
  pub fn span(&self) -> &Span {
    &self.span
  }

  pub(crate) unsafe fn get_string_unchecked(&self) -> String {
    match self.token() {
      Token::Identifier(s) => s.to_owned(),
      Token::LiteralText(s) => s.to_owned(),
      _ => unreachable!(),
    }
  }

  pub(crate) unsafe fn get_i64_unchecked(&self) -> i64 {
    match self.token() {
      Token::LiteralInteger(i) => i.to_owned(),
      _ => unreachable!(),
    }
  }

  pub(crate) unsafe fn get_bool_unchecked(&self) -> bool {
    match self.token() {
      Token::False => false,
      Token::True => true,
      _ => unreachable!(),
    }
  }
}

impl Debug for LocatableToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let arr = self.span.expand();
    write!(
      f,
      "Token[{:?}]:{},{}:{},{}",
      self.token, arr.0, arr.1, arr.2, arr.3,
    )
  }
}

impl Display for LocatableToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let arr = self.span.expand();
    write!(
      f,
      "{:?}:{},{}:{},{}",
      self.token, arr.0, arr.1, arr.2, arr.3,
    )
  }
}

impl Locatable for LocatableToken {
  fn get_span(&self) -> &Span {
    &self.span
  }
}
