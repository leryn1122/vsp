use std::fmt::Debug;
use std::ops::Deref;

use vsp_ast::ast::expr::Expression;
use vsp_ast::ast::stmt::Statement;
use vsp_span::Span;
use vsp_support::debug_println;

use crate::parser::token::LocatableToken;
use crate::parser::token::TokenStream;
use crate::token::TokenType;

pub(crate) type Input<'ctx> = TokenStream;

#[derive(Debug)]
pub(crate) enum ParseResult<T> {
  Matched(T, Span),
  Failed,
}

impl<T> ParseResult<T> {}

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
  input: &'ctx Input,
  state: &mut ParseState<'ctx>,
  pos: usize,
) -> ParseResult<Statement> {
  parse_stmts(input, state, pos)
}

pub(crate) fn parse_stmts<'ctx>(
  input: &'ctx Input,
  state: &mut ParseState<'ctx>,
  pos: usize,
) -> ParseResult<Statement> {
  parse_stmt(input, state, pos)
}

pub(crate) fn parse_stmt<'ctx>(
  input: &'ctx Input,
  state: &mut ParseState<'ctx>,
  pos: usize,
) -> ParseResult<Statement> {
  ParseResult::Failed
}

pub(crate) fn parse_expr<'ctx>(
  input: &'ctx Input,
  state: &mut ParseState<'ctx>,
  pos: usize,
) -> ParseResult<Expression> {
  let mut choice_res = ParseResult::Failed;
  let mut case = 0;

  let current = input.get(pos);
  if current.is_none() {
    return ParseResult::Failed;
  }

  loop {
    choice_res = match case {
      0 => parse_literal(input, state, pos.to_owned()),
      1 => parse_unary(input, state, pos.to_owned()),
      2 => parse_binary(input, state, pos.to_owned()),
      _ => ParseResult::Failed,
    };

    debug_println!("{:?}", &choice_res);

    match choice_res {
      ParseResult::Matched(t, pos) => return ParseResult::Matched(t, pos),
      ParseResult::Failed => {
        if case > 0 {
          return ParseResult::Failed;
        } else {
          case += 1;
          continue;
        }
      }
    }
  }
}

/// Parse literal value.
pub(crate) fn parse_literal<'ctx>(
  input: &'ctx Input,
  state: &mut ParseState<'ctx>,
  pos: usize,
) -> ParseResult<Expression> {
  let current = input.get(pos);
  if current.is_none() {
    return ParseResult::Failed;
  }
  let current = current.unwrap();

  match current.token().token_type() {
    TokenType::Identifier => ParseResult::Matched(
      Expression::Identifier(unsafe { current.get_string_unchecked() }),
      current.span().clone(),
    ),
    TokenType::LiteralText => ParseResult::Matched(
      Expression::LiteralString(unsafe { current.get_string_unchecked() }),
      current.span().clone(),
    ),
    TokenType::LiteralInteger => ParseResult::Matched(
      Expression::LiteralInteger(unsafe { current.get_i64_unchecked() }),
      current.span().clone(),
    ),
    _ => ParseResult::Failed,
  }
}

pub(crate) fn parse_unary<'ctx>(
  input: &'ctx Input,
  state: &mut ParseState<'ctx>,
  pos: usize,
) -> ParseResult<Expression> {
  ParseResult::Failed
}

pub(crate) fn parse_binary<'ctx>(
  input: &'ctx Input,
  state: &mut ParseState<'ctx>,
  pos: usize,
) -> ParseResult<Expression> {
  ParseResult::Failed
}

pub(crate) fn parse_infix<'ctx>(
  input: &'ctx Input,
  state: &mut ParseState<'ctx>,
  pos: usize,
  lowest: Precedence,
) -> ParseResult<Expression> {
  let current = input.get(pos);
  if current.is_none() {
    return ParseResult::Failed;
  }

  let left: Option<Expression> = None;
  let right: Option<Expression> = None;

  ParseResult::Failed
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
    let mut tokens = lexer.tokenize("1 + 1").unwrap();

    let mut state = ParseState::new();
    match parse_expr(&mut tokens, &mut state, 0) {
      ParseResult::Matched(t, pos) => {
        debug_println!("{:?}", t);
      }
      ParseResult::Failed => {
        debug_println!("Failed")
      }
    }
  }
}
