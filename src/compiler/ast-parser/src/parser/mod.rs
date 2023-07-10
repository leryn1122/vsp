use vsp_ast::ast::expr::BinaryOp;
use vsp_ast::ast::expr::Expression;
use vsp_ast::ast::stmt::Statement;
use vsp_ast::ast::stmt::StatementBlock;
use vsp_error::VspResult;
use vsp_support::debug_println;
use vsp_support::ptr::make_shared_ptr;

use crate::parser::context::TokenContext;
use crate::parser::state::ParserState;
use crate::parser::token::LocatableToken;
use crate::parser::token::TokenStream;
use crate::token::Token;

mod context;
mod state;
pub mod token;

pub struct DefaultParser;

impl DefaultParser {
  pub fn parse(&mut self, tokens: TokenStream) -> VspResult<()> {
    let mut ctx = TokenContext::from_str(&tokens);
    let mut state = ParserState::new();

    while let Some(token) = ctx.peek() {
      match token.token() {
        Token::RBrace => {
          parse_stmt_block(&mut state, &mut ctx);
          ctx.next();
        }
        _ => state.add_token(ctx.next().unwrap()),
      }
    }

    state.stmts().iter().for_each(|s| println!("{:?}", s));
    Ok(())
  }
}

/// Consume all the expressions.
pub fn parse_stmt(state: &mut ParserState, ctx: &mut TokenContext) {
  if state.exprs().is_empty() && state.tokens().is_empty() {
    state.add_stmt(Statement::NoOp);
    return;
  }

  #[cfg(debug_assertions)]
  {
    state.print();
  }

  while let Some(token) = state.pop_token() {
    #[cfg(debug_assertions)]
    {
      debug_println!("Current = {:?}", token);
      state.print();
    }

    match token.token() {
      Token::Dot => {}
      Token::Comma => {}
      Token::Colon => {}
      Token::SemiColon => {}
      Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Percentage => {
        parse_binary_op(state, ctx, token);
      }

      // Token::LParenthesis => {}
      // Token::RParenthesis => {}
      // Token::LBracket => {}
      // Token::RBracket => {}
      Token::LBrace => {
        state.add_token(token);
        break;
      }
      // Token::RBrace => {}
      // Token::Less => {}
      // Token::Greater => {}
      // Token::LessEqual => {}
      // Token::GreaterEqual => {}
      // Token::Equal => {}
      // Token::NotEqual => {}
      // Token::Assigment => {}
      // Token::At => {}
      // Token::Not => {}
      // Token::And => {}
      // Token::Or => {}
      // Token::Xor => {}
      // Token::Question => {}
      // Token::SQuote => {}
      // Token::DQuote => {}
      // Token::TQuote => {}
      // Token::Arrow => {}
      // Token::DArrow => {}
      // Token::DColon => {}
      // Token::As => {}
      // Token::Async => {}
      // Token::Await => {}
      // Token::Break => {}
      // Token::Const => {}
      // Token::Continue => {}
      // Token::Else => {}
      // Token::Enum => {}
      // Token::False => {}
      // Token::Func => {}
      // Token::For => {}
      // Token::If => {}
      // Token::Impl => {}
      // Token::Import => {}
      // Token::In => {}
      // Token::Int => {}
      // Token::Let => {}
      // Token::Loop => {}
      // Token::Module => {}
      // Token::Optional => {}
      // Token::Public => {}
      // Token::Ref => {}
      Token::Return => parse_return(state, ctx),
      // Token::Static => {}
      // Token::Struct => {}
      // Token::Super => {}
      // Token::Trait => {}
      // Token::True => {}
      // Token::Type => {}
      // Token::Union => {}
      // Token::Unsafe => {}
      // Token::Use => {}
      // Token::Var => {}
      // Token::Where => {}
      // Token::While => {}
      // Token::Self_ => {}
      Token::Identifier(s) => state.add_expr(Expression::Identifier(s.clone())),
      Token::LiteralText(s) => state.add_expr(Expression::LiteralString(s.clone())),
      Token::LiteralInteger(num) => state.add_expr(Expression::LiteralInteger(num.clone())),
      Token::LiteralFloat(_) => {}
      _ => {
        debug_println!("parse_stmt = {:?}", token);
        unimplemented!()
      }
    }
  }
}

/// Consume all the statements.
pub fn parse_stmt_block(state: &mut ParserState, ctx: &mut TokenContext) {
  state.add_stmt_blocks(StatementBlock::new());

  #[cfg(debug_assertions)]
  {
    state.print();
  }

  while let Some(token) = state.pop_token() {
    #[cfg(debug_assertions)]
    {
      debug_println!("Current = {:?}", token);
      state.print();
    }

    if token.token().is_stmt_terminator() {
      parse_stmt(state, ctx);
    } else if token.token().is_stmt_block_starter() {
    } else {
    }
  }
}

pub fn parse_expr_if_absent(state: &mut ParserState, ctx: &mut TokenContext) {
  if let Some(token) = state.pop_token() {
    if token.token().is_to_expression() {
      let _ = token.try_into().map(|e| state.add_expr(e)).unwrap();
    } else {
      todo!()
    }
  } else {
    return;
  }
}

/// Consume to 2 expressions and 1 token.
pub fn parse_binary_op(state: &mut ParserState, ctx: &mut TokenContext, token: LocatableToken) {
  let rhs = state.pop_expr().unwrap();
  parse_expr_if_absent(state, ctx);
  let lhs = state.pop_expr().unwrap();

  let op = match token.token() {
    Token::Plus => BinaryOp::Add,
    Token::Minus => BinaryOp::Subtract,
    Token::Asterisk => BinaryOp::Multiply,
    Token::Slash => BinaryOp::Division,
    _ => unreachable!(),
  };
  let mut expr = Expression::Binary(op, make_shared_ptr(lhs), make_shared_ptr(rhs));
  state.add_expr(expr);
}

/// Consume 1 expressions.
pub fn parse_return(state: &mut ParserState, ctx: &mut TokenContext) {
  let expr = state.pop_expr().unwrap();
  state.add_stmt(Statement::Return(Some(expr)));
}

impl Token {
  #[inline]
  pub fn is_stmt_block_starter(&self) -> bool {
    self == &Token::LBrace
  }

  #[inline]
  pub fn is_stmt_starter(&self) -> bool {
    self == &Token::Let || self == &Token::Var
  }

  #[inline]
  pub fn is_stmt_terminator(&self) -> bool {
    self == &Token::Colon
  }

  #[inline]
  pub fn is_to_expression(&self) -> bool {
    matches!(
      self,
      Token::False
        | Token::True
        | Token::Self_
        | Token::Identifier(_)
        | Token::LiteralText(_)
        | Token::LiteralInteger(_)
        | Token::LiteralFloat(_)
    )
  }
}

impl TryInto<Expression> for LocatableToken {
  type Error = ();

  fn try_into(self) -> Result<Expression, Self::Error> {
    debug_println!("Try into {:?}", self);
    match self.token() {
      Token::False => Ok(Expression::LiteralBoolean(false)),
      Token::True => Ok(Expression::LiteralBoolean(true)),
      // Token::Self_ => {},
      // Token::Identifier(_) => {},
      Token::LiteralText(s) => Ok(Expression::LiteralString(s.clone())),
      Token::LiteralInteger(val) => Ok(Expression::LiteralInteger(*val)),
      // Token::LiteralFloat(_) => {},
      _ => Err(()),
    }
  }
}
