use combine::between;
use combine::choice;
use combine::many;
use combine::many1;
use combine::none_of;
use combine::optional;
use combine::parser::char::char;
use combine::parser::char::digit;
use combine::parser::char::letter;
use combine::parser::char::spaces;
use combine::parser::char::string;
use combine::satisfy;
use combine::sep_by;
use combine::token;
use combine::ParseError;
use combine::Parser;
use combine::Stream;

pub type VspToken = vsp_token::Token;

/// A function, of great importance in lexical analysis, splits the character stream into sequences
/// of lexemes.
/// Lexemes are divided into several categories below,
/// - Keyword
/// - Punctuation
/// - Identifier
/// - Literal text
/// - Literal numeric
pub fn tokenize<Input>() -> impl Parser<Input, Output = Vec<VspToken>>
where
  Input: Stream<Token = char> + Copy,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  sep_by(token_parser(), char(' ').or(char('\n'))).skip(optional(char('\n')))
}

/// The choice candidate parser is designed to be listed in the given order.
pub fn token_parser<Input>() -> impl Parser<Input, Output = VspToken>
where
  Input: Stream<Token = char> + Copy,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  choice((
    // Order is of great importance.
    keyword(),
    punctuation(),
    identifier(),
    literal_text(),
    literal_integer(),
  ))
}

pub fn keyword<Input>() -> impl Parser<Input, Output = VspToken>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  many1::<String, _, _>(letter())
    .map(|s| match VspToken::mapping_non_literal_token(s.as_ref()) {
      Some(token) => token,
      None => VspToken::Identifier(s),
    })
    .expected("valid keyword")
}

pub fn punctuation<Input>() -> impl Parser<Input, Output = VspToken>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  many1::<String, _, _>(is_not_alpha_num())
    .map(|s| match VspToken::mapping_non_literal_token(s.as_ref()) {
      Some(token) => token,
      None => VspToken::Identifier(s),
    })
    .expected("valid keyword")
}

pub fn is_not_alpha_num<Input>() -> impl Parser<Input, Output = char, PartialState = ()>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  satisfy(|ch: char| !ch.is_alphanumeric() && !ch.is_whitespace()).expected("letter")
}

/// Regular expression pattern as `[a-zA-Z][a-zA-Z0-9_]+`, combined with one identifier starter and
/// identifier successor.
pub fn identifier<Input>() -> impl Parser<Input, Output = VspToken>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  (
    identifier_starter(),
    optional(many::<String, _, _>(identifier_successor())),
  )
    .map(|(starter, successor)| {
      if let Some(successor) = successor {
        VspToken::Identifier(format!("{}{}", starter, successor))
      } else {
        VspToken::Identifier(format!("{}", starter))
      }
    })
    .expected("valid identifier successor")
}

/// Determine the ***starter*** of the identifier.
fn identifier_starter<Input>() -> impl Parser<Input, Output = char, PartialState = ()>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  satisfy(|ch: char| ch.is_alphabetic()).expected("valid identifier starter")
}

/// Determine the ***successor*** of the identifier.
fn identifier_successor<Input>() -> impl Parser<Input, Output = char, PartialState = ()>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  satisfy(|ch: char| ch.is_alphanumeric() || ch == '_').expected("valid identifier successor")
}

/// Parse the literal text quote by the double quote character.
pub fn literal_text<Input>() -> impl Parser<Input, Output = VspToken>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  let literal_text =
    || between(char('"'), char('"'), many1(none_of(Some('"')))).map(VspToken::LiteralText);
  literal_text().skip(spaces()).expected("valid literal text")
}

/// Parse the literal integer, positive or negative.
pub(crate) fn literal_integer<Input>() -> impl Parser<Input, Output = VspToken>
where
  Input: Stream<Token = char> + Copy,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  let digits = || many1(digit()).map(|s: String| s.parse::<i64>().unwrap());
  digits().map(VspToken::LiteralInteger)
}

#[allow(dead_code)]
//noinspection RsWrongGenericArgumentsNumber,RsWrongGenericArgumentsOrder
fn sign<Input>() -> impl Parser<Input, Output = Option<char>>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  optional(choice((token('+'), token('-'))))
}

#[allow(dead_code)]
fn digits<Input>() -> impl Parser<Input, Output = String>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  many1(digit()).map(|digits: String| digits)
}

/// Parse literal boolean value.
/// It's not obtained.
#[allow(dead_code)]
pub(crate) fn boolean<Input>() -> impl Parser<Input, Output = VspToken>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  choice((string("true"), string("false"))).map(|s| VspToken::mapping_non_literal_token(s).unwrap())
}

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
      .parse("public func main ( ) -> int { print return 0 ; }")
      .map(|(t, _)| println!("{:?}", t));
  }

  #[test]
  pub fn test_identifier() {
    let mut parser = sep_by(identifier(), space()).map(|tokens: Vec<VspToken>| {
      println!(
        "{:?}",
        <Vec<VspToken> as AsRef<Vec<VspToken>>>::as_ref(&tokens)
      );
    });
    let result = parser.parse("t token token_ token token123 token_123 token123token");
  }

  #[test]
  pub fn test_literal_text() {
    let result = literal_text()
      .skip(spaces())
      .parse("\"Lorem ipsum dolor sit amet, consectetur adipisicing elit\"   ")
      .map(|(t, _)| println!("{:?}", t));
  }
}
