use std::str::Chars;

use vsp_span::span::Position;

use crate::combinator::basic::Optional;
use crate::combinator::basic::Some;
use crate::combinator::conjunct::And;
use crate::combinator::conjunct::AndThen;
use crate::combinator::conjunct::Chain;
use crate::combinator::conjunct::Filter;
use crate::combinator::conjunct::Many;
use crate::combinator::conjunct::Map;
use crate::combinator::conjunct::Or;
use crate::combinator::satisfy::satisfy;
use crate::parser::convert::IntoParser;
use crate::parser::primitive::char;

pub mod convert;
pub mod primitive;

#[derive(Clone, Debug)]
pub struct ParseState<'a> {
  pub(crate) src: Chars<'a>,
  pos:            Position,
  offset:         usize,
}

impl<'a> ParseState<'a> {
  pub fn new(src: &'a str) -> Self {
    Self {
      src:    src.chars(),
      pos:    Position::start(),
      offset: 1,
    }
  }

  pub fn as_str(&self) -> &'a str {
    self.src.as_str()
  }
}

impl<'a> Iterator for ParseState<'a> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    let ch = self.src.next()?;
    self.pos = match ch {
      '\n' => self.pos.line_feed(),
      _ => self.pos.forward(),
    };
    self.offset += 1;
    Some(ch)
  }
}

/// Parser on a stateful set.
pub trait Parser<S> {
  type Target;

  fn parse<'a>(&self, state: &mut S) -> Option<Self::Target>;
}

pub trait ParserExt<S>: Parser<S> {
  fn or<U>(self, other: U) -> Or<Self, U::Parser>
  where
    Self: Sized,
    U: IntoParser<S, Target = Self::Target>,
  {
    Or::new(self, other.into_parser())
  }

  fn and<U>(self, other: U) -> And<Self, U::Parser>
  where
    Self: Sized,
    U: IntoParser<S>,
  {
    And::new(self, other.into_parser())
  }

  fn map<B, F>(self, functor: F) -> Map<Self, F>
  where
    Self: Sized,
    F: Fn(Self::Target) -> B,
  {
    Map::new(self, functor)
  }

  fn filter<F>(self, functor: F) -> Filter<Self, F>
  where
    Self: Sized,
    F: Fn(&Self::Target) -> bool,
  {
    Filter::new(self, functor)
  }

  fn and_then<U, F>(self, functor: F) -> AndThen<Self, F>
  where
    Self: Sized,
    U: IntoParser<S>,
    F: Fn(Self::Target) -> U,
  {
    AndThen::new(self, functor)
  }

  fn chain<U>(self, other: U) -> Chain<Self, U::Parser>
  where
    Self: Parser<S> + Sized,
    U: IntoParser<S>,
    Self::Target: IntoIterator,
    U::Target: IntoIterator<Item = <Self::Target as IntoIterator>::Item>,
  {
    Chain::new(self, other.into_parser())
  }

  fn many(self) -> Many<Self>
  where
    Self: Sized,
  {
    Many::new(self)
  }

  fn some(self) -> Some<Self>
  where
    Self: Sized,
  {
    Some::new(self)
  }

  fn option(self) -> Optional<Self>
  where
    Self: Sized,
  {
    Optional::new(self)
  }

  fn info(&self, message: &str) {
    println!("{}", message);
  }
}

impl<S, P: Parser<S>> ParserExt<S> for P {}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::parser::primitive::char;

  #[test]
  pub fn main() {
    let mut src = ParseState::new("Hello world");
    let res = char('H').parse(&mut src);
    assert!(res.is_some());
  }
}
