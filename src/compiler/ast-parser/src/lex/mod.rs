use core::iter::Peekable;
use core::str::Chars;

use smallvec::SmallVec;
use vsp_error::VspResult;
use vsp_span::Position;
use vsp_span::Span;

use crate::parser::token::LocatableToken;
use crate::parser::token::TokenStream;
use crate::token::mapping_non_literal_token;
use crate::token::Token;

pub struct DefaultLexer;

impl DefaultLexer {
  pub fn tokenize(&mut self, str: &str) -> VspResult<TokenStream> {
    let mut ctx = CharContext::from(str);
    let mut tokens = vec![];
    'core: while let Some(c) = ctx.next() {
      if c.is_whitespace() {
        continue 'core;
      }
      match c {
        '.' => punctuation_handler(&mut tokens, &mut ctx, Token::Dot),
        ',' => punctuation_handler(&mut tokens, &mut ctx, Token::Comma),
        ';' => punctuation_handler(&mut tokens, &mut ctx, Token::Colon),
        '+' => punctuation_handler(&mut tokens, &mut ctx, Token::Plus),
        '*' => punctuation_handler(&mut tokens, &mut ctx, Token::Asterisk),
        '/' => punctuation_handler(&mut tokens, &mut ctx, Token::Slash),
        '%' => punctuation_handler(&mut tokens, &mut ctx, Token::Percentage),
        '(' => punctuation_handler(&mut tokens, &mut ctx, Token::LParenthesis),
        ')' => punctuation_handler(&mut tokens, &mut ctx, Token::RParenthesis),
        '[' => punctuation_handler(&mut tokens, &mut ctx, Token::LBracket),
        ']' => punctuation_handler(&mut tokens, &mut ctx, Token::RBracket),
        '{' => punctuation_handler(&mut tokens, &mut ctx, Token::LBrace),
        '}' => punctuation_handler(&mut tokens, &mut ctx, Token::RBrace),
        '@' => punctuation_handler(&mut tokens, &mut ctx, Token::At),
        '^' => punctuation_handler(&mut tokens, &mut ctx, Token::Xor),
        '?' => punctuation_handler(&mut tokens, &mut ctx, Token::Question),
        '0'..='9' => readout_numeric(&mut tokens, &mut ctx),
        '"' => readout_literal_string(&mut tokens, &mut ctx),
        _ => readout_symbol(&mut tokens, &mut ctx),
      };
    }

    Ok(tokens)
  }
}

pub(crate) type CharIterator<'a> = Peekable<Chars<'a>>;

/// Context for iterating over a char sequence holds cached current / previous characters and
/// information on the position and location.
struct CharContext<'a> {
  chars:    CharIterator<'a>,
  pos:      Position,
  last_pos: Position,
  loc:      usize,
  curr:     Option<char>,
  prev:     Option<char>,
}

impl<'a> CharContext<'a> {
  pub fn from(str: &'a str) -> Self {
    Self {
      chars:    str.chars().peekable(),
      pos:      Position::new(),
      last_pos: Position::new(),
      loc:      0,
      curr:     None,
      prev:     None,
    }
  }

  #[inline]
  pub fn location(&mut self) -> usize {
    self.loc
  }

  #[inline]
  pub fn curr(&mut self) -> Option<char> {
    self.curr
  }

  #[inline]
  pub fn curr_unchecked(&mut self) -> char {
    self.curr.unwrap()
  }

  #[inline]
  pub fn curr_pos(&self) -> &Position {
    &self.pos
  }

  #[inline]
  pub fn last_pos(&self) -> &Position {
    &self.last_pos
  }

  pub fn peek(&mut self) -> Option<&char> {
    self.chars.peek()
  }
}

impl<'a> Iterator for CharContext<'a> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    let res = self.chars.next();
    if res.is_none() {
      return res.to_owned();
    }

    self.last_pos = self.pos;

    if let Some(c) = res {
      match c {
        '\n' => self.pos.line_feed(),
        '\r' => self.pos,
        _ => self.pos.forward(),
      };
    }

    self.loc += 1;
    self.prev = self.curr;
    self.curr = res;

    res.to_owned()
  }
}

impl<'a> AsRef<CharIterator<'a>> for CharContext<'a> {
  fn as_ref(&self) -> &CharIterator<'a> {
    &self.chars
  }
}

/// Common punctuation handler for .
fn punctuation_handler(tokens: &mut TokenStream, ctx: &mut CharContext, expected: Token) {
  let token = LocatableToken::new(
    expected,
    Span::range(ctx.last_pos().clone(), ctx.curr_pos().clone()),
  );
  tokens.push(token);
}

fn readout_numeric(tokens: &mut TokenStream, ctx: &mut CharContext) {
  let digit = ctx.curr_unchecked().to_digit(10).unwrap().into();
  let token = Token::LiteralInteger(digit);
  let span = Span::at(ctx.curr_pos().clone());
  let token = LocatableToken::new(token, span);
  tokens.push(token);
}

/// Read out the symbol
fn readout_symbol(tokens: &mut TokenStream, ctx: &mut CharContext) {
  let mut buf: SmallVec<[char; 1 << 4]> = SmallVec::new();
  let start = ctx.last_pos().clone();
  buf.push(ctx.curr_unchecked());
  while let Some(&c) = ctx.peek() {
    // If not a valid identifier successor, collect the chars in buffer.
    if is_identifier_successor(c) {
      buf.push(c);
      ctx.next();
    } else {
      let token = buf.iter().collect::<String>();
      let token = token.as_str();
      let token = read_keyword_or_identifier(token);
      let span = Span::range(start, ctx.curr_pos().clone());

      if let Some(token) = token {
        tokens.push(LocatableToken::new(token, span));
        return;
      }
    }
  }
}

fn readout_literal_string(tokens: &mut TokenStream, ctx: &mut CharContext) {
  let mut buf: Vec<char> = Vec::with_capacity(1 << 4);
  let start = ctx.last_pos().clone();
  while let Some(&c) = ctx.peek() {
    if c != '"' {
      buf.push(c);
      ctx.next();
    } else {
      let token = Token::LiteralText(buf.iter().collect::<String>());
      let span = Span::range(start, ctx.curr_pos().clone());
      tokens.push(LocatableToken::new(token, span));
      ctx.next();
      return;
    }
  }
}

fn read_keyword_or_identifier(s: &str) -> Option<Token> {
  if let Some(token) = mapping_non_literal_token(s) {
    Some(token)
  } else if is_valid_identifier(s) {
    Some(Token::Identifier(s.to_string()))
  } else {
    None
  }
}

fn is_valid_identifier(_s: &str) -> bool {
  // TODO:
  true
}

/// True if it is a valid identifier starter
#[allow(unused)]
fn is_identifier_starter(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}

/// True if it is a valid identifier successor.
fn is_identifier_successor(c: char) -> bool {
  c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_lexer() {
    let mut lex = DefaultLexer {};
    let tokens = lex.tokenize("public func main() { return 0; }").unwrap();
    for token in tokens {
      println!("{:?}", token);
    }
  }
}
