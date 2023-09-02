use vsp_ast::ast::expr::Expression;
use vsp_ast::ast::AST;
use vsp_error::VspResult;
use vsp_support::debug_println;

use crate::parser::combine::CombinatorParser;
use crate::parser::token::LocatableToken;
use crate::parser::token::TokenStream;
use crate::token::Token;

mod context;
mod state;
pub mod token;

/// Kind of parser.
#[repr(u8)]
pub enum ParserKind {
  /// Traditional parser.
  Traditional,
  /// Parser combinator.
  Combinator,
}

/// ```rust
/// use vsp_ast_parser::parser::token::TokenStream;
/// use vsp_ast_parser::parser::ASTFactory;
/// use vsp_ast_parser::parser::ParserKind;
///
/// let mut tokens: TokenStream = vec![];
/// let mut parser = ASTFactory::create_parser(ParserKind::Default);
/// let _ = parser.parse(tokens);
/// ```
pub struct ASTFactory;

impl ASTFactory {
  /// Create the specific AST parser by kind.
  pub fn create_parser(kind: ParserKind) -> Box<dyn ASTParser> {
    use self::ParserKind::*;
    match kind {
      Combinator => Box::new(CombinatorParser {}),
      _ => Box::new(TraditionalParser {}),
    }
  }

  pub fn create_default_parser() -> Box<dyn ASTParser> {
    Box::new(TraditionalParser {})
  }
}

pub trait ASTParser {
  fn parse(&mut self, tokens: TokenStream) -> VspResult<AST>;
}

pub struct TraditionalParser;

impl ASTParser for TraditionalParser {
  fn parse(&mut self, tokens: TokenStream) -> VspResult<AST> {
    todo!()
  }
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
    self == &Token::SemiColon
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
      Token::LiteralText(s) => Ok(Expression::LiteralString(s.to_owned())),
      Token::LiteralInteger(val) => Ok(Expression::LiteralInteger(*val)),
      // Token::LiteralFloat(_) => {},
      _ => Err(()),
    }
  }
}

enum TerminatorKind {
  /// `]`
  Array,
  /// `}`
  Block,
}

pub(crate) mod combine {
  use vsp_ast::ast::AST;
  use vsp_error::VspResult;

  use crate::parser::token::TokenStream;
  use crate::parser::ASTParser;

  pub struct CombinatorParser;

  impl ASTParser for CombinatorParser {
    fn parse(&mut self, tokens: TokenStream) -> VspResult<AST> {
      todo!()
    }
  }
}
