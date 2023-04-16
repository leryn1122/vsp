use crate::parser::primitive::CharacterParser;
use crate::parser::primitive::StringParser;
use crate::parser::ParseState;
use crate::parser::Parser;

pub trait IntoParser<S> {
  type Parser: Parser<S, Target = Self::Target>;
  type Target;

  fn into_parser(self) -> Self::Parser;
}

impl<S, P> IntoParser<S> for P
where
  P: Parser<S>,
{
  type Parser = P;
  type Target = P::Target;

  fn into_parser(self) -> Self::Parser {
    self
  }
}

impl<'a> IntoParser<ParseState<'a>> for char {
  type Parser = CharacterParser;
  type Target = char;

  fn into_parser(self) -> Self::Parser {
    CharacterParser::new(self)
  }
}

impl<'s> IntoParser<ParseState<'s>> for &str {
  type Parser = StringParser;
  type Target = &'s str;

  fn into_parser(self) -> Self::Parser {
    StringParser::new(self)
  }
}
