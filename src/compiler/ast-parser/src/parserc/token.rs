use std::str::FromStr;

use combine::between;
use combine::many;
use combine::many1;
use combine::none_of;
use combine::parser::char::alpha_num;
use combine::parser::char::char;
use combine::parser::char::letter;
use combine::parser::char::spaces;
use combine::parser::choice::or;
use combine::parser::combinator::map;
use combine::parser::error::expected;
use combine::parser::sequence::skip;
use combine::position;
use combine::satisfy;
use combine::token;
use combine::ParseError;
use combine::Parser;
use combine::Stream;
use vsp_token::Keyword;
use vsp_token::Punctuator;
use vsp_token::TokenType;

/// A function, of great importance in lexical analysis, splits the character stream into sequences
/// of lexemes.
pub fn tokenize<Input>() -> impl Parser<Input, Output = TokenType>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  let token = keyword()
    .or(identifier())
    .or(literal_text())
    // .or(punctuator())
    .skip(spaces());
  token
}

pub(crate) fn keyword<Input>() -> impl Parser<Input, Output = TokenType>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  many::<String, _, _>(letter())
    .map(|s| match Keyword::from_str(s.as_ref()) {
      Ok(k) => TokenType::Keyword(k),
      Err(s) => TokenType::Identifier(s),
    })
    .expected("valid keyword")
}

// pub(crate) fn punctuator<Input>() -> impl Parser<Input, Output = TokenType>
// where
//   Input: Stream<Token = char>,
//   Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//   let punctuator = || {
//     if let Some(p) = Punctuator::from_string("") {
//       p
//     } else {
//       Punctuator::Question
//     }
//   };
//   punctuator
// }

pub(crate) fn literal_text<Input>() -> impl Parser<Input, Output = TokenType>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  let string = || {
    between(token('"'), token('"'), many1(none_of(Some('"'))))
      .map(|s: String| TokenType::LiteralText(s.into()))
  };
  string().skip(spaces()).expected("valid literal text")
}

/// Determine the ***starter*** of the identifier.
pub(crate) fn identifier_starter<Input>() -> impl Parser<Input, Output = char, PartialState = ()>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  satisfy(|ch: char| ch.is_alphabetic()).expected("valid identifier starter")
}

/// Determine the ***successor*** of the identifier.
pub(crate) fn identifier_successor<Input>() -> impl Parser<Input, Output = char, PartialState = ()>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  satisfy(|ch: char| ch.is_alphanumeric() || ch == '_').expected("valid identifier successor")
}

/// Regular expression pattern as `[a-zA-Z][a-zA-Z0-9_]+`.
pub fn identifier<Input>() -> impl Parser<Input, Output = TokenType>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  (
    identifier_starter(),
    many::<String, _, _>(identifier_successor()),
  )
    .map(|(starter, successor)| TokenType::Identifier(format!("{}{}", starter, successor)))
    .expected("valid identifier successor")
}

// /// Determine the ***successor*** of the identifier.
// pub(crate) fn preserve_word<Input>() -> impl Parser<Input, Output = TokenType>
// where
//   Input: Stream<Token = char>,
//   Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
// }

#[cfg(test)]
mod tests {
  use std::println;

  use combine::parser::char::alpha_num;
  use combine::parser::char::space;
  use combine::sep_by;
  use combine::Parser;
  use vsp_token::TokenType;

  use super::*;

  #[test]
  pub fn test_tokenize() {
    let result = tokenize()
      .parse(
        "\
      public func main() -> int {\
        print(\"Hello world!!\");
        return 0;
      }",
      )
      .map(|(t, _)| println!("{:?}", t));
  }

  #[test]
  pub fn test_identifier() {
    let identifier = identifier();
    let mut parser = sep_by(identifier, space()).map(|tokens: Vec<TokenType>| {
      println!(
        "{:?}",
        <Vec<TokenType> as AsRef<Vec<TokenType>>>::as_ref(&tokens)
      );
    });
    let result = parser.parse("t token token_ token token123 token_123 token123token");
  }

  #[test]
  pub fn test_literal_text() {
    let mut literal_text = literal_text().skip(spaces());
    let result = literal_text
      .parse("\"Lorem ipsum dolor sit amet, consectetur adipisicing elit\"   ")
      .map(|(t, _)| println!("{:?}", t));
  }
}
