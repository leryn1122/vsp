use regex::Match;
use regex::Regex;

use crate::parser::ParseState;
use crate::parser::Parser;

pub fn regex(regex: &str) -> RegexParser {
  RegexParser {
    delegate: Regex::new(regex).unwrap(),
  }
}

pub struct RegexParser {
  delegate: Regex,
}

impl RegexParser {
  pub fn new(regex: &str) -> Result<Self, regex::Error> {
    Regex::new(regex).map(|delegate| Self { delegate: delegate })
  }

  pub(crate) fn delegate(&self) -> &Regex {
    &self.delegate
  }

  pub(crate) fn unwrap(self) -> Regex {
    self.delegate
  }
}

impl From<Regex> for RegexParser {
  fn from(regex: Regex) -> Self {
    Self { delegate: regex }
  }
}

impl<'s> Parser<ParseState<'s>> for RegexParser {
  type Target = &'s str;

  fn parse<'a>(&self, state: &mut ParseState<'s>) -> Option<Self::Target> {
    let src = state.as_str();
    match self.delegate.find(src) {
      Some(m) if m.start() == 0 => {
        state.take(m.end()).for_each(|_| {});
        Some(&src[0..m.end()])
      }
      _ => None,
    }
  }
}
