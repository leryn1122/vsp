use std::fmt::Debug;

use smallvec::SmallVec;
use vsp_ast::ast::expr::BinaryOp;
use vsp_ast::ast::expr::Expression;
use vsp_ast::ast::stmt::Statement;
use vsp_ast::ast::stmt::StatementBlock;
use vsp_ast::ast::DeclNode;
use vsp_ast::ast::Declaration;

use crate::parser::token::LocatableToken;
use crate::parser::token::TokenStream;
use crate::token::Token;
use crate::token::TokenType;

pub(crate) type Input<'ctx> = TokenStream;

#[derive(Debug)]
pub enum ParseResult<T> {
  Matched(T, (usize, usize)),
  Failed,
}

impl<T> ParseResult<T> {
  pub const fn is_err(&self) -> bool {
    matches!(self, ParseResult::Failed)
  }

  pub fn unwrap(&self) -> (T, (usize, usize))
  where
    T: Clone,
  {
    match self {
      ParseResult::Matched(value, range) => (value.clone(), range.to_owned()),
      ParseResult::Failed => panic!(),
    }
  }
}

/// The lower the precedence enumeration lies, the higher precedence the token has.
#[repr(u8)]
pub(crate) enum Precedence {
  Lowest = 0,
  Assign,
  Logic,
  Ternary,
  Compare,
  Sum,
  Product,
  Prefix,
  Call,
  Index,
  Dot,
}

impl LocatableToken {
  fn into_precedence(&self) -> Precedence {
    match self.token() {
      Token::Equal => Precedence::Assign,
      Token::Not | Token::And | Token::Or | Token::Xor => Precedence::Logic,
      // _ => Precedence::Ternary,
      Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual => Precedence::Compare,
      // _ => Precedence::Sum,
      // _ => Precedence::Product,
      // _ => Precedence::Prefix,
      // _ => Precedence::Call,
      // _ => Precedence::Index,
      Token::Dot => Precedence::Dot,
      _ => Precedence::Lowest,
    }
  }
}

#[inline]
fn is_left_marker(token: &LocatableToken) -> bool {
  match token.token() {
    Token::SemiColon | Token::LParenthesis | Token::LBracket | Token::LBrace => true,
    _ => false,
  }
}

#[inline]
fn is_right_marker(token: &LocatableToken) -> bool {
  match token.token() {
    Token::RParenthesis | Token::RBracket | Token::RBrace => true,
    _ => false,
  }
}

fn is_unary_op(token: &LocatableToken) -> bool {
  match token.token() {
    Token::Asterisk | Token::Not | Token::Minus => true,
    _ => false,
  }
}

fn is_binary_op(token: &LocatableToken) -> bool {
  match token.token() {
    Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Percentage => true,
    _ => false,
  }
}

fn into_precedence(token: &LocatableToken) -> Precedence {
  match token.token() {
    Token::Equal => Precedence::Assign,
    Token::Not | Token::And | Token::Or | Token::Xor => Precedence::Logic,
    // _ => Precedence::Ternary,
    Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual => Precedence::Compare,
    // _ => Precedence::Sum,
    // _ => Precedence::Product,
    // _ => Precedence::Prefix,
    // _ => Precedence::Call,
    // _ => Precedence::Index,
    Token::Dot => Precedence::Dot,
    _ => Precedence::Lowest,
  }
}

pub(crate) struct ParseState<'ctx> {
  _phantom: std::marker::PhantomData<&'ctx ()>,
}

impl<'ctx> ParseState<'ctx> {
  pub fn new() -> ParseState<'ctx> {
    Self {
      _phantom: std::marker::PhantomData,
    }
  }
}

fn peek(tokens: &TokenStream, pos: usize) -> Option<&LocatableToken> {
  tokens.get(pos + 1)
}

fn get(tokens: &TokenStream, pos: usize) -> Option<&LocatableToken> {
  tokens.get(pos)
}

pub(crate) fn parse<'ctx>(
  state: &mut ParseState<'ctx>,
  input: &'ctx Input,
  range: (usize, usize),
) -> ParseResult<Statement> {
  let mut decls: SmallVec<[Declaration; 8]> = SmallVec::new();

  // parse_stmts(state, input, pos, range);
  ParseResult::Failed
}

pub(crate) fn parse_stmts<'ctx>(
  state: &mut ParseState<'ctx>,
  input: &'ctx Input,
  range: (usize, usize),
) -> ParseResult<Statement> {
  let mut stmts: SmallVec<[Statement; 16]> = SmallVec::new();
  let (stmt, range) = parse_stmt(state, input, range).unwrap();

  let stmts = stmts.to_vec();
  ParseResult::Matched(
    Statement::Block(Box::new(StatementBlock::from(stmts))),
    range.to_owned(),
  )
}

pub(crate) fn parse_stmt<'ctx>(
  state: &mut ParseState<'ctx>,
  input: &'ctx Input,
  range: (usize, usize),
) -> ParseResult<Statement> {
  let mut i = range.0;
  while i <= range.1 {
    let current = input.get(i).unwrap();
    let next = input.get(i + 1);
    if matches!(current.token(), Token::Return) {
      let range = (i + 1, range.1);
      let (expr, range) = parse_literal(state, input, range).unwrap();
      i = range.1 + 1;
    }
  }
  ParseResult::Failed
}

pub(crate) fn parse_expr<'ctx>(
  state: &mut ParseState<'ctx>,
  input: &'ctx Input,
  range: (usize, usize),
) -> ParseResult<Expression> {
  // Stack of parsed expressions
  let mut exprs: SmallVec<[Expression; 8]> = SmallVec::new();

  let mut i = range.0;
  while i <= range.1 {
    let current = input.get(i).unwrap();
    let next = input.get(i + 1);
    // Literal
    if current.token().into_u8() >= crate::token::LITERAL_TYPE_ID {
      let range = (i, i);
      let (expr, range) = parse_literal(state, input, range).unwrap();
      exprs.push(expr);
      i += 1;
      continue;
    }
    // Binary Ops
    else if is_binary_op(&current) {
      if next.is_some() && !exprs.is_empty() {
        let range = (i, range.1);
        let binops = parse_binary(
          state,
          input,
          range,
          current.into_precedence(),
          exprs.pop().unwrap(),
        );
        if binops.is_err() {
          return ParseResult::Failed;
        } else {
          let binops = binops.unwrap();
          exprs.push(binops.0);
          i = binops.1 .1 + 1;
        }
      } else {
        return ParseResult::Failed;
      }
      i += 1;
      continue;
    }
  }
  debug_assert!(exprs.len() == 1, "expression stack should be empty");
  ParseResult::Matched(exprs.pop().unwrap(), (range.0, range.1))
}

/// Parse literal value unchecked.
pub(crate) fn parse_literal<'ctx>(
  _state: &mut ParseState<'ctx>,
  input: &'ctx Input,
  range: (usize, usize),
) -> ParseResult<Expression> {
  debug_assert!(range.0 == range.1, "range must be equal");
  let current = input.get(range.0).unwrap();
  match current.token().token_type() {
    TokenType::Identifier => ParseResult::Matched(
      Expression::Identifier(unsafe { current.get_string_unchecked() }),
      range,
    ),
    TokenType::LiteralText => ParseResult::Matched(
      Expression::LiteralString(unsafe { current.get_string_unchecked() }),
      range,
    ),
    TokenType::LiteralInteger => ParseResult::Matched(
      Expression::LiteralInteger(unsafe { current.get_i64_unchecked() }),
      range,
    ),
    _ => ParseResult::Failed,
  }
}

pub(crate) fn parse_binary<'ctx>(
  state: &mut ParseState<'ctx>,
  input: &'ctx Input,
  range: (usize, usize),
  precedence: Precedence,
  left: Expression,
) -> ParseResult<Expression> {
  let token = input.get(range.0).unwrap();
  let op = match token.token() {
    Token::Plus => BinaryOp::Add,
    Token::Minus => BinaryOp::Subtract,
    Token::Asterisk => BinaryOp::Multiply,
    Token::Slash => BinaryOp::Division,
    _ => return ParseResult::Failed,
  };
  let right = parse_expr(state, input, (range.0 + 1, range.1));
  if right.is_err() {
    return ParseResult::Failed;
  }
  let right = right.unwrap();
  let binops = Expression::Binary(op, Box::new(left), Box::new(right.0));
  ParseResult::Matched(binops, right.1)
}

#[cfg(test)]
mod tests {
  use vsp_support::debug_println;

  use crate::lex::DefaultLexer;
  use crate::parser::state::parse_expr;
  use crate::parser::state::ParseResult;
  use crate::parser::state::ParseState;

  #[test]
  pub fn test() {
    let mut lexer = DefaultLexer {};
    let mut tokens = lexer.tokenize("1 * 2 - 3").unwrap();
    let mut end = tokens.len() - 1;

    let mut state = ParseState::new();
    match parse_expr(&mut state, &mut tokens, (0, end).clone()) {
      ParseResult::Matched(t, range) => {
        debug_println!("result = {:?}", t);
      }
      ParseResult::Failed => {
        debug_println!("Failed")
      }
    }
  }
}
