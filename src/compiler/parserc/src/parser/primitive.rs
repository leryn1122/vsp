use regex::Regex;
use vsp_ast::ast::types::PrimitiveType::Int64;

use crate::combinator::regex::RegexParser;
use crate::parser::ParseState;
use crate::parser::Parser;

#[derive(Debug, Clone, Copy)]
pub struct CharacterParser(char);

impl CharacterParser {
  pub fn new(ch: char) -> Self {
    Self(ch)
  }
}

impl<'a> Parser<ParseState<'a>> for CharacterParser {
  type Target = char;

  fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
    match state.next() {
      None => None,
      Some(ch) => {
        if ch == self.0 {
          Some(ch)
        } else {
          None
        }
      }
    }
  }
}

pub fn char(ch: char) -> CharacterParser {
  CharacterParser::new(ch)
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct IntegerParser;

impl IntegerParser {
  pub fn new() -> Self {
    Self
  }
}

impl<'a> Parser<ParseState<'a>> for IntegerParser {
  type Target = isize;

  fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
    let regex = Regex::new("^\\d+").unwrap();
    let parser = RegexParser::from(regex);
    if let Some(s) = parser.parse(state) {
      return s.parse::<isize>().ok();
    }
    None
  }
}

#[derive(Clone, Debug)]
pub struct StringParser {
  pub(crate) temp: String,
}

impl<'s> StringParser {
  pub fn new(str: &'s str) -> Self {
    Self {
      temp: str.to_owned(),
    }
  }
}

impl<'a> Parser<ParseState<'a>> for StringParser {
  type Target = &'a str;

  fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
    let src = state.as_str();
    if let Some(0) = src.find(&self.temp) {
      state.take(self.temp.len()).for_each(|_| {});
      Some(&src[0..self.temp.len()])
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_integer_parser() {
    let mut state = ParseState::new("12138");
    assert_eq!(
      IntegerParser::new().parse(&mut state).unwrap(),
      12138 as isize
    );
  }
}
