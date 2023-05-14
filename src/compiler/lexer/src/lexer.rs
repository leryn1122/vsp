use core::iter::Peekable;
use core::str::Chars;

use anyhow::anyhow;
use vsp_span::span::Position;
use vsp_span::span::Span;
use vsp_token::Keyword;
use vsp_token::Punctuator;
use vsp_token::Token;
use vsp_token::TokenStream;
use vsp_token::TokenType;

pub type CharIter<'a> = Peekable<Chars<'a>>;

/// Cursor / iterator over characters.
pub struct DefaultLexer<'a> {
  chars: CharIter<'a>,
  #[allow(dead_code)]
  prev:  char,
}

impl<'a> DefaultLexer<'a> {
  pub fn from_str(str: &'a str) -> Self {
    Self {
      chars: str.chars().peekable(),
      prev:  '\0',
    }
  }

  /// Accept a character iterator. Iterate and resolve character one by one.
  /// - Literal string
  /// - Numeric values
  ///
  /// Luckily, for most punctuators, it was immediately resolved as the mapped
  /// token without the context.
  pub fn tokenize(&mut self) -> anyhow::Result<TokenStream> {
    let mut tokens = Vec::new();
    let mut pos = Position::new();
    let iter = self.chars.by_ref();
    let handler = |p, pos: &mut Position| {
      let token = Token::new(TokenType::Punctuator(p), Span::single_char(pos.clone()));
      pos.forward();
      token
    };
    while let Some(c) = iter.next() {
      match c {
        '\r' | '\n' => readout_whitespace(iter, &mut pos).unwrap(),
        ' ' | '\t' => {
          pos.forward();
        }
        '.' => tokens.push(handler(Punctuator::Dot, &mut pos)),
        ',' => tokens.push(handler(Punctuator::Comma, &mut pos)),
        ';' => tokens.push(handler(Punctuator::Colon, &mut pos)),
        ':' => tokens.push(handler(Punctuator::SemiColon, &mut pos)),
        '*' => tokens.push(handler(Punctuator::Asterisk, &mut pos)),
        '%' => tokens.push(handler(Punctuator::Percentage, &mut pos)),
        '(' => tokens.push(handler(Punctuator::LParenthesis, &mut pos)),
        ')' => tokens.push(handler(Punctuator::RParenthesis, &mut pos)),
        '[' => tokens.push(handler(Punctuator::LBracket, &mut pos)),
        ']' => tokens.push(handler(Punctuator::RBracket, &mut pos)),
        '{' => tokens.push(handler(Punctuator::LBrace, &mut pos)),
        '}' => tokens.push(handler(Punctuator::RBrace, &mut pos)),
        '@' => tokens.push(handler(Punctuator::At, &mut pos)),
        '"' => tokens.push(read_literal_text(c, iter, &mut pos).unwrap()),
        '0'..='9' => tokens.push(read_numeric(c, iter, &mut pos).unwrap()),
        _ => tokens.push(read_symbol(c, iter, &mut pos).unwrap()),
      }
    }
    Ok(tokens)
  }
}

#[allow(unused_variables)]
fn readout_whitespace(iter: &mut CharIter, pos: &mut Position) -> anyhow::Result<()> {
  pos.line_feed();
  Ok(())
}

/// Read symbol, that is identifier or keyword, from given chars.
fn read_symbol(c: char, iter: &mut CharIter, pos: &mut Position) -> anyhow::Result<Token> {
  let mut buff: Vec<char> = Vec::with_capacity(8);
  let start = pos.clone();
  buff.push(c);
  while let Some(&c) = iter.peek() {
    if is_identifier_successor(c) {
      buff.push(c);
      iter.next();
      pos.forward();
    } else {
      let token = buff.iter().collect::<String>();
      let token = token.as_str();
      let token = read_keyword_or_identifier(token);
      let span = Span::range(start, pos.clone());
      return Ok(Token::new(token, span));
    }
  }
  Err(anyhow!("Unexpected token"))
}

/// Read numeric from given chars.
#[allow(unused_variables)]
fn read_literal_text(c: char, iter: &mut CharIter, pos: &mut Position) -> anyhow::Result<Token> {
  let mut buff: Vec<char> = Vec::with_capacity(1 << 6);
  let start = pos.clone();
  while let Some(&c) = iter.peek() {
    if c != '"' {
      buff.push(c);
      iter.next();
      pos.forward();
    } else {
      let text = buff.iter().collect::<String>();
      let token = TokenType::LiteralText(text);
      let span = Span::range(start, pos.clone());
      iter.next();
      return Ok(Token::new(token, span));
    }
  }
  Err(anyhow!("Unexpected token"))
}

/// Read numeric from given chars.
/// FIXME only integer could be handled normally.
fn read_numeric(c: char, _iter: &mut CharIter, pos: &mut Position) -> anyhow::Result<Token> {
  let token = TokenType::LiteralNumeric(c as i64);
  let span = Span::single_char(pos.clone());
  let token = Token::new(token, span);
  Ok(token)
}

/// True if the char is valid at the ***starter*** of the identifier.
#[allow(unused)]
pub(crate) fn is_identifier_starter(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}

/// True if the char is valid as the ***successor*** of the identifier.
pub(crate) fn is_identifier_successor(c: char) -> bool {
  c.is_alphanumeric() || c == '_'
}

fn read_keyword_or_identifier(s: &str) -> TokenType {
  if let Ok(keyword) = Keyword::from_str(s) {
    TokenType::Keyword(keyword)
  } else {
    TokenType::Identifier(s.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tokenize() -> Result<(), ()> {
    let source = "\
public func int main() {
  print(\"Hello world!!\");
  return 0;
}
";
    let mut lexer = DefaultLexer::from_str(source);
    let tokens = lexer.tokenize().unwrap();
    for token in tokens {
      println!("{:?}", token);
    }
    Ok(())
  }
}
