use std::fmt::Debug;
use std::fmt::Formatter;

use vsp_span::span::Span;
use vsp_span::Locatable;

/// The final result of the lexical analysis, which are transferred to the AST parser.
pub type TokenStream = Vec<Token>;

/// A token in lexical analysis.
#[rustfmt::skip]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Token {
  //============================================================================//
  //  Punctuation
  //============================================================================//

  /*  `.`   */Dot = 0,
  /*  `,`   */Comma,
  /*  `;`   */Colon,
  /*  `:`   */SemiColon,
  /*  `+`   */Plus,
  /*  `-`   */Minus,
  /*  `*`   */Asterisk,
  /*  `/`   */Slash,
  /*  `%`   */Percentage,
  /*  `(`   */LParenthesis,
  /*  `)`   */RParenthesis,
  /*  `[`   */LBracket,
  /*  `]`   */RBracket,
  /*  `{`   */LBrace,
  /*  `}`   */RBrace,
  /*  `<`   */Less,
  /*  `>`   */Greater,
  /*  `<=`  */LessEqual,
  /*  `>=`  */GreaterEqual,
  /*  `==`  */Equal,
  /*  `!=`  */NotEqual,
  /*  `=`   */Assigment,
  /*  `@`   */At,
  /*  `!`   */Not,
  /*  `&&`  */And,
  /*  `||`  */Or,
  /*  `^`   */Xor,
  /*  `?`   */Question,
  /*  `'`   */SQuote,
  /*  `"`   */DQuote,
  /*  `"""` */TQuote,
  /*  `->`  */Arrow,
  /*  `=>`  */DArrow,
  /*  `::`  */DColon,
  
  //============================================================================//
  //  Keywords
  //============================================================================//

  // A-G
  As = 1 << 6,
  Async,
  Await,
  Break,
  Const,
  Continue,
  Else,
  Enum,
  False,
  Func,
  For,

  // H-N
  If,
  Impl,
  Import,
  In,
  Int,
  Let,
  Loop,
  Module,

  // O-T
  Optional,
  Public,
  Ref,
  Return,
  Static,
  Struct,
  Super,
  True,
  Type,

  // U-Z
  Union,
  Unsafe,
  Use,
  Var,
  Where,
  While,

  Self_, // for Self
  
  //============================================================================//
  //  Literals
  //============================================================================//
  #[allow(deprecated)]
  Identifier(String) = std::u8::MAX - 4,
  LiteralText(String),
  LiteralInteger(i64),
  LiteralFloat(String),
}

impl Token {
  /// Mapping the literal string to the corresponding non-literal token.
  /// If the return values is `None`, try to
  #[rustfmt::skip]
  pub fn mapping_non_literal_token(s: &str) -> Option<Token> {
    match s {
      "."                => Some(Self::Dot),
      ","                => Some(Self::Comma),
      ";"                => Some(Self::Colon),
      ":"                => Some(Self::SemiColon),
      "+"                => Some(Self::Plus),
      "-"                => Some(Self::Minus),
      "*"                => Some(Self::Asterisk),
      "/"                => Some(Self::Slash),
      "%"                => Some(Self::Percentage),
      "("                => Some(Self::LParenthesis),
      ")"                => Some(Self::RParenthesis),
      "["                => Some(Self::LBracket),
      "]"                => Some(Self::RBracket),
      "{"                => Some(Self::LBrace),
      "}"                => Some(Self::RBrace),
      "<"                => Some(Self::Less),
      ">"                => Some(Self::Greater),
      "<="               => Some(Self::LessEqual),
      ">="               => Some(Self::GreaterEqual),
      "=="               => Some(Self::Equal),
      "!="               => Some(Self::NotEqual),
      "="                => Some(Self::Assigment),
      "@"                => Some(Self::At),
      "!"                => Some(Self::Not),
      "&&"               => Some(Self::And),
      "||"               => Some(Self::Or),
      "^"                => Some(Self::Xor),
      "?"                => Some(Self::Question),
      "->"               => Some(Self::Arrow),
      "=>"               => Some(Self::DArrow),
      "::"               => Some(Self::DColon),
      "'"                => Some(Self::SQuote),
      "\""               => Some(Self::DQuote),
      "\"\"\""           => Some(Self::TQuote),

      "as"               => Some(Self::As),
      "async"            => Some(Self::Async),
      "await"            => Some(Self::Await),
      "break"            => Some(Self::Break),
      "const"            => Some(Self::Const),
      "continue"         => Some(Self::Continue),
      "else"             => Some(Self::Else),
      "enum"             => Some(Self::Enum),
      "false"            => Some(Self::False),
      "func"             => Some(Self::Func),
      "for"              => Some(Self::For),
      "if"               => Some(Self::If),
      "impl"             => Some(Self::Impl),
      "import"           => Some(Self::Import),
      "in"               => Some(Self::In),
      "int"              => Some(Self::Int),
      "let"              => Some(Self::Let),
      "loop"             => Some(Self::Loop),
      "module"           => Some(Self::Module),
      "optional"         => Some(Self::Optional),
      "public"           => Some(Self::Public),
      "ref"              => Some(Self::Ref),
      "return"           => Some(Self::Return),
      "static"           => Some(Self::Static),
      "struct"           => Some(Self::Struct),
      "super"            => Some(Self::Super),
      "true"             => Some(Self::True),
      "type"             => Some(Self::Type),
      "union"            => Some(Self::Union),
      "unsafe"           => Some(Self::Unsafe),
      "use"              => Some(Self::Use),
      "var"              => Some(Self::Var),
      "where"            => Some(Self::Where),
      "while"            => Some(Self::While),
      "self"             => Some(Self::Self_),

      _ => None,
    }
  }

  /// Internal implementation, don't use.
  #[allow(clippy::wrong_self_convention)]
  fn into_u8(&self) -> u8 {
    unsafe { *(self as *const Self as *const u8) }
  }

  /// Use ordinal number to determine which range it belongs to.
  pub fn token_type(&self) -> TokenType {
    let ord = self.into_u8();
    if ord < Self::Dot.into_u8() {
      match self {
        Token::Identifier(_) => TokenType::Identifier,
        Token::LiteralText(_) => TokenType::LiteralText,
        Token::LiteralInteger(_) => TokenType::LiteralInteger,
        Token::LiteralFloat(_) => TokenType::LiteralFloat,
        _ => unreachable!("Unsupported token type"),
      }
    } else if ord < Self::As.into_u8() {
      TokenType::Punctuation
    } else {
      TokenType::Keyword
    }
  }
}

#[repr(usize)]
pub enum TokenType {
  Punctuation,
  Keyword,
  Identifier,
  LiteralText,
  LiteralInteger,
  LiteralFloat,
}

/// Base of numeric literal value.
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Base {
  Binary = 2,
  Octal = 8,
  /// Default numeric base.
  Decimal = 10,
  Hexadecimal = 16,
}

/// A locatable token with span, as its start and end location in the source codes.
pub struct LocatableToken {
  pub token: Token,
  pub span:  Span,
}

impl Debug for LocatableToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let arr = self.span.expand_as_array();
    write!(
      f,
      "Token = [{:?}]:{},{}:{},{}",
      self.token, arr[0], arr[1], arr[2], arr[3],
    )
  }
}

impl Locatable for LocatableToken {
  fn get_span(&self) -> &Span {
    &self.span
  }
}
