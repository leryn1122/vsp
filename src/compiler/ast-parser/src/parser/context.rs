use crate::token::{LocatableToken, TokenStream};
use std::iter::Peekable;
use std::slice::Iter;

pub(crate) type TokenIterator<'a> = Peekable<Iter<'a, LocatableToken>>;

///
pub struct TokenContext<'a> {
  tokens: TokenIterator<'a>,
  loc: usize,
  curr: Option<LocatableToken>,
  prev: Option<LocatableToken>,
}

impl<'a> TokenContext<'a> {
  pub fn from_str(tokens: &'a TokenStream) -> Self {
    Self {
      tokens: tokens.iter().peekable(),
      loc: 0,
      curr: None,
      prev: None,
    }
  }

  #[inline]
  pub fn location(&mut self) -> usize {
    self.loc
  }

  #[inline]
  pub fn current(&mut self) -> Option<LocatableToken> {
    self.curr.clone()
  }

  #[inline]
  pub fn current_unchecked(&mut self) -> &LocatableToken {
    self.curr.as_ref().unwrap()
  }

  pub fn peek(&mut self) -> Option<&LocatableToken> {
    self.tokens.peek().copied()
  }
}

impl<'a> Iterator for TokenContext<'a> {
  type Item = LocatableToken;

  fn next(&mut self) -> Option<Self::Item> {
    let res = self.tokens.next();
    if res.is_some() {
      self.loc += 1;
      self.prev = self.curr.clone();
      self.curr = res.cloned();
    }
    res.cloned()
  }
}
