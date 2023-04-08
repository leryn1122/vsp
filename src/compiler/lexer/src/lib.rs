use std::str::FromStr;

pub mod lexer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
  Keyword(Keyword),
  Identifier(String),
  LiteralText(String),
  LiteralNumeric(String),
  Punctuator(Punctuator),
  EOF,
}

impl FromStr for Token {
  type Err = anyhow::Error;

  fn from_str(_s: &str) -> Result<Self, Self::Err> {
    todo!()
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Punctuator {
  //============================================================================//

  // Separator
  /// `.`
  Dot,
  /// `,`
  Comma,
  /// `;`
  Colon,
  /// `:`
  SemiColon,
  /// `+`
  Plus,
  /// `-`
  Minus,
  /// `*`
  Asterisk,
  /// `/`
  Slash,
  /// `%`
  Percentage,

  /// `(`
  LParenthesis,
  /// `)`
  RParenthesis,
  /// `[`
  LBracket,
  /// `]`
  RBracket,
  /// `{`
  LBrace,
  /// `}`
  RBrace,

  // Comparison
  /// `<`
  Less,
  /// `>`
  Greater,
  /// `<=`
  LessEqual,
  /// `>=`
  GreaterEqual,
  /// `==`
  Equal,
  /// `!=`
  NotEqual,
  /// `=`
  Assigment,

  /// `@`
  At,

  // Logic Operators
  /// `!`
  Not,
  /// `&&`
  And,
  /// `||`
  Or,
  /// `^`
  Xor,

  // Bit Operators
  // TODO

  // Quotation
  /// `?`
  Question,
  /// `'`
  SQuote,
  /// `"`
  DQuote,
  /// `"""`
  TQuote,

  // Lambda
  /// `->`
  Arrow,
  /// `=>`
  DArrow,
  /// `::`
  DColon,
  //============================================================================//
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
  //============================================================================//

  // A-G
  As,
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
  //============================================================================//
}

impl FromStr for Keyword {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "as" => Ok(Keyword::As),
      "async" => Ok(Keyword::Async),
      "await" => Ok(Keyword::Await),
      "break" => Ok(Keyword::Break),
      "const" => Ok(Keyword::Const),
      "continue" => Ok(Keyword::Continue),
      "else" => Ok(Keyword::Else),
      "enum" => Ok(Keyword::Enum),
      "false" => Ok(Keyword::False),
      "func" => Ok(Keyword::Func),
      "for" => Ok(Keyword::For),
      "if" => Ok(Keyword::If),
      "impl" => Ok(Keyword::Impl),
      "import" => Ok(Keyword::Import),
      "in" => Ok(Keyword::In),
      "int" => Ok(Keyword::Int),
      "let" => Ok(Keyword::Let),
      "loop" => Ok(Keyword::Loop),
      "module" => Ok(Keyword::Module),
      "optional" => Ok(Keyword::Optional),
      "public" => Ok(Keyword::Public),
      "ref" => Ok(Keyword::Ref),
      "return" => Ok(Keyword::Return),
      "static" => Ok(Keyword::Static),
      "struct" => Ok(Keyword::Struct),
      "super" => Ok(Keyword::Super),
      "true" => Ok(Keyword::True),
      "type" => Ok(Keyword::Type),
      "union" => Ok(Keyword::Union),
      "unsafe" => Ok(Keyword::Unsafe),
      "use" => Ok(Keyword::Use),
      "var" => Ok(Keyword::Var),
      "where" => Ok(Keyword::Where),
      "while" => Ok(Keyword::While),
      &_ => Err(()),
    }
  }
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
