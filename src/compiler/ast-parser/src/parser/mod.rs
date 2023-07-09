mod context;
mod state;

use crate::parser::context::TokenContext;
use crate::parser::state::ParserState;
use crate::token::{LocatableToken, TokenStream};
use vsp_ast::ast::expr::ExpressionKind;
use vsp_ast::ast::stmt::{Statement, StatementKind};
use vsp_error::VspResult;
use vsp_support::debug_println;
use vsp_token::Token;

pub struct DefaultParser;

impl DefaultParser {
  pub fn parse(&mut self, tokens: TokenStream) -> VspResult<()> {
    let mut ctx = TokenContext::from_str(&tokens);
    let mut state = ParserState::new();

    while let Some(token) = ctx.next() {
      match token.token() {
        Token::RBrace => {
          self.parse_stmt_block(&mut state, &mut ctx);
        }
        Token::Colon => self.parse_stmt(&mut state, &mut ctx),
        _ => state.add_token(token),
      }
    }

    state.stmts().iter().for_each(|s| println!("{:?}", s));
    Ok(())
  }

  pub fn parse_stmt(&mut self, state: &mut ParserState, ctx: &mut TokenContext) {
    if state.exprs().is_empty() && state.tokens().is_empty() {
      state.add_stmt(StatementKind::NoOp);
      return;
    }

    while let Some(token) = state.pop_token() {
      debug_println!("parse_stmt = {:?}", token);
      match token.token() {
        Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Percentage => {
          self.parse_binop(state, ctx);
        }
        Token::Return => {
          self.parse_return(state, ctx);
        }
        // Token::Identifier(_) => {}
        // Token::LiteralText(_) => {}
        Token::LiteralInteger(num) => state.add_expr(ExpressionKind::LiteralInteger(*num)),
        // Token::LiteralFloat(_) => {}
        _ => {
          debug_println!("{:?}", token)
        }
      }
    }
  }

  pub fn parse_stmt_block(&mut self, state: &mut ParserState, ctx: &mut TokenContext) {
    debug_println!("parse_stmt_block");
    // state.add_expected_type()
  }

  pub fn parse_binop(&mut self, state: &mut ParserState, ctx: &mut TokenContext) {
    debug_println!("parse_binop");
    let rhs = state.pop_expr().unwrap();
    let ops = state.pop_expr().unwrap();
    let lhs = state.pop_expr().unwrap();
  }

  pub fn parse_return(&mut self, state: &mut ParserState, ctx: &mut TokenContext) {
    let expr = state.pop_expr().unwrap();
    state.add_stmt(StatementKind::Return(Some(expr)));
  }
}

fn is_starter(token: &LocatableToken) {}

/// True if token is a valid terminator of a statement or expression.
fn is_terminator(token: &LocatableToken) -> bool {
  token.token() == &Token::Colon
}

#[cfg(test)]
mod tests {
  use crate::parser::DefaultParser;

  #[test]
  fn test_tokenize() {
    let mut lex = crate::lex::DefaultLexer {};
    let mut tokens = lex.tokenize("{ return 0; }").unwrap();
    // for token in &tokens {
    //   println!("{:?}", token);
    // }

    let mut parser = DefaultParser {};
    let result = parser.parse(tokens);
  }
}
