use core::str::Chars;
use std::iter::Peekable;
use std::str::FromStr;

use vsp_error::VspResult;

use crate::Keyword;
use crate::Punctuator;
use crate::Token;

type CharIter<'a> = Peekable<Chars<'a>>;

pub fn tokenize(iter: &mut CharIter) -> VspResult<Vec<Token>> {
  let mut tokens: Vec<Token> = Vec::new();
  while let Some(c) = iter.next() {
    match c {
      ' ' | '\t' | '\r' | '\n' => {
        readout_whitespace(iter)?;
      }
      '.' => {
        tokens.push(Token::Punctuator(Punctuator::Dot));
      }
      ',' => {
        tokens.push(Token::Punctuator(Punctuator::Comma));
      }
      ';' => {
        tokens.push(Token::Punctuator(Punctuator::Colon));
      }
      ':' => {
        tokens.push(Token::Punctuator(Punctuator::SemiColon));
      }
      '*' => {
        tokens.push(Token::Punctuator(Punctuator::Asterisk));
      }
      '%' => {
        tokens.push(Token::Punctuator(Punctuator::Percentage));
      }
      '(' => {
        tokens.push(Token::Punctuator(Punctuator::LParenthesis));
      }
      ')' => {
        tokens.push(Token::Punctuator(Punctuator::RParenthesis));
      }
      '[' => {
        tokens.push(Token::Punctuator(Punctuator::LBracket));
      }
      ']' => {
        tokens.push(Token::Punctuator(Punctuator::RBracket));
      }
      '{' => {
        tokens.push(Token::Punctuator(Punctuator::LBrace));
      }
      '}' => {
        tokens.push(Token::Punctuator(Punctuator::RBrace));
      }
      '"' => {
        tokens.push(read_literal_text(c, iter)?);
      }
      '0'..='9' => {
        tokens.push(read_numeric(c, iter)?);
      }
      _ => {
        tokens.push(read_symbol(c, iter)?);
      }
    }
  }
  Ok(tokens)
}

/// Read numeric from given chars.
#[allow(unused_variables)]
fn read_literal_text(c: char, iter: &mut CharIter) -> VspResult<Token> {
  let mut buff: Vec<char> = Vec::with_capacity(1 << 6);
  while let Some(&c) = iter.peek() {
    if c != '"' {
      buff.push(c);
      iter.next();
    } else {
      let text = buff.iter().collect::<String>();
      iter.next();
      iter.next();
      return Ok(Token::LiteralText(text));
    }
  }
  Ok(Token::EOF)
}

/// Read numeric from given chars.
fn read_numeric(c: char, iter: &mut CharIter) -> VspResult<Token> {
  Ok(Token::LiteralNumeric(c.to_string()))
}

/// Read symbol, that is identifier or keyword, from given chars.
fn read_symbol(c: char, iter: &mut CharIter) -> VspResult<Token> {
  let mut buff: Vec<char> = Vec::with_capacity(8);
  buff.push(c);
  while let Some(&c) = iter.peek() {
    if is_identifier_successor(c) {
      buff.push(c);
      iter.next();
    } else {
      let token = buff.iter().collect::<String>();
      let token = token.as_str();
      iter.next();
      return Ok(read_keyword_or_identifier(token));
    }
  }
  Ok(Token::EOF)
}

fn readout_whitespace(iter: &mut CharIter) -> VspResult {
  Ok(())
}

fn is_whitespace(c: char) -> bool {
  matches!(c, ' ' | '\t' | '\n' | '\r')
}

/// True if the char is valid at the <b>start</b> of the identifier.
#[allow(unused)]
pub(crate) fn is_identifier_start(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}

/// True if the char is valid as the <b>successor</b> of the identifier.
#[allow(unused)]
pub(crate) fn is_identifier_successor(c: char) -> bool {
  c.is_alphanumeric() || c == '_'
}

#[allow(unused)]
pub fn strip_shebang(_text: &str) -> Option<usize> {
  None
}

fn read_keyword_or_identifier(s: &str) -> Token {
  if let Ok(keyword) = Keyword::from_str(s) {
    Token::Keyword(keyword)
  } else {
    Token::Identifier(s.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_tokenize() {
    let source = "\
public func int main() {
  print(\"Hello world!!\");
  return 0;
}
";
    let mut char_iter = source.chars().peekable();
    let expected: Vec<Token> = tokenize(&mut char_iter).unwrap();
    let result = vec![
      Token::Keyword(Keyword::Public),
      Token::Keyword(Keyword::Func),
      Token::Keyword(Keyword::Int),
      Token::Identifier("main".to_string()),
      Token::Punctuator(Punctuator::RParenthesis),
      Token::Punctuator(Punctuator::LBrace),
      Token::Identifier("print".to_string()),
      Token::LiteralText("Hello world!!".to_string()),
      Token::Punctuator(Punctuator::Colon),
      Token::Keyword(Keyword::Return),
      Token::LiteralNumeric("0".to_string()),
      Token::Punctuator(Punctuator::Colon),
      Token::Punctuator(Punctuator::RBrace),
    ];
    assert_eq!(result.len(), expected.len());
    for i in 1..expected.len() {
      assert_eq!(*result.get(i).unwrap(), *expected.get(i).unwrap())
    }
  }
}
