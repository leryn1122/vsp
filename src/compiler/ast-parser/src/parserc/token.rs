use combine::between;
use combine::many;
use combine::many1;
use combine::none_of;
use combine::parser::char::letter;
use combine::parser::char::spaces;
use combine::satisfy;
use combine::token;
use combine::ParseError;
use combine::Parser;
use combine::Stream;
use vsp_token::Token;

/// A function, of great importance in lexical analysis, splits the character stream into sequences
/// of lexemes.
pub fn tokenize<Input>() -> impl Parser<Input, Output = Token>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  keyword()
    // .or(punctuator())
    .or(identifier())
    .or(literal_text())
    // .or(literal_numeric())
    .skip(spaces())
}

pub(crate) fn keyword<Input>() -> impl Parser<Input, Output = Token>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  many::<String, _, _>(letter())
    .map(|s| match Token::mapping_non_literal_token(s.as_ref()) {
      Some(token) => token,
      None => Token::Identifier(s),
    })
    .expected("valid keyword")
}

// pub(crate) fn punctuator<Input>() -> impl Parser<Input, Output = TokenType>
// where
//   Input: Stream<Token = char>,
//   Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//   let punctuator = many::<String, _, _>(is_not_alpha_num()).map(|| {});
//   token("!=")
//     .or(token("{"))
//     .or(token("}"))
//     .map(|s| match Keyword::from_str(s) {
//       Ok(_) => {}
//       Err(_) => {}
//     });
//   punctuator
// }

pub fn is_not_alpha_num<Input>() -> impl Parser<Input, Output = char, PartialState = ()>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  satisfy(|ch: char| !ch.is_alphanumeric() && !ch.is_whitespace()).expected("letter")
}

/// Regular expression pattern as `[a-zA-Z][a-zA-Z0-9_]+`.
pub fn identifier<Input>() -> impl Parser<Input, Output = Token>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  (
    identifier_starter(),
    many::<String, _, _>(identifier_successor()),
  )
    .map(|(starter, successor)| Token::Identifier(format!("{}{}", starter, successor)))
    .expected("valid identifier successor")
}

pub(crate) fn literal_text<Input>() -> impl Parser<Input, Output = Token>
where
  Input: Stream<Token = char>,
  Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
  let string =
    || between(token('"'), token('"'), many1(none_of(Some('"')))).map(Token::LiteralText);
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

// pub fn integer<Input>() -> impl Parser<Input, Output = TokenType>
// where
//   Input: Stream<Token = char>,
//   Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
//   F: Extend<P::Output> + Default,
// {
//   (
//     many1(digit()).expected("?"),
//     optional(char('.')),
//     optional(recognize(many1(digit()))),
//   )
//     .map(|(integer, dot, fraction)| {
//       let s = format!("{}.{}", integer, fraction.unwrap_or("0".to_string()));
//       if dot.is_some() || fraction.is_some() {
//         TokenType::LiteralNumeric(s.parse::<i64>().unwrap())
//       } else {
//         TokenType::LiteralNumeric(s.parse::<i64>().unwrap())
//       }
//     })
// }

//
// /// Determine the literal number.
// pub(crate) fn literal_numeric<Input>() -> impl Parser<Input, Output = TokenType>
// where
//   Input: Stream<Token = char>,
//   Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//   let integer = (optional(string("-")), many1(digit())).map(|(sign, digits)| {
//     let integer = digits.to_string();
//     let integer = digits.parse::<i64>().unwrap();
//     if sign.is_some() {
//       TokenType::LiteralNumeric(-integer)
//     } else {
//       TokenType::LiteralNumeric(integer)
//     }
//   });
//   // .map(|(sign, digits)| {
//   //   let number = digits.to_string();
//   //   let number = number.parse::<i64>().expect("valid integer");
//   //   if sign.is_some() {
//   //     TokenType::LiteralNumeric(-number)
//   //   } else {
//   //     TokenType::LiteralNumeric(number)
//   //   }
//   // });
//   integer
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
      public func main  int \
        print \"Hello world\"
        return
      ",
      )
      .map(|(t, _)| println!("{:?}", t));
  }

  #[test]
  pub fn test_identifier() {
    let mut parser = sep_by(identifier(), space()).map(|tokens: Vec<Token>| {
      println!("{:?}", <Vec<Token> as AsRef<Vec<Token>>>::as_ref(&tokens));
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

  // #[test]
  // pub fn test_literal_numeric() {
  //   let mut parser = literal_numeric().map(|tokens: Vec<TokenType>| {
  //     println!(
  //       "{:?}",
  //       <Vec<TokenType> as AsRef<Vec<TokenType>>>::as_ref(&tokens)
  //     );
  //   });
  //   let result = parser.parse("123 0 -0 -12138");
  // }
}
