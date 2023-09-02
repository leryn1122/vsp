use core::fmt::Debug;

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
  /*  `:`   */Colon,
  /*  `;`   */SemiColon,
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
  Int,
  Let,
  Loop,
  Module,

  // O-T
  Public,
  Ref,
  Return,
  Static,
  Struct,
  Trait,
  True,
  Type,

  // U-Z
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
  /// Internal implementation, don't use.
  #[allow(clippy::wrong_self_convention)]
  fn into_u8(&self) -> u8 {
    unsafe { *(self as *const Self as *const u8) }
  }

  /// Use ordinal number to determine which range it belongs to.
  pub fn token_type(&self) -> TokenType {
    let ord = self.into_u8();
    // Use branch prediction.
    if ord > Token::Self_.into_u8() {
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

/// Mapping the literal string to the corresponding non-literal token.
/// If the return values is `None`, try to
#[rustfmt::skip]
pub fn mapping_non_literal_token(s: &str) -> Option<Token> {
  match s {
    "." => Some(Token::Dot),
    "," => Some(Token::Comma),
    ":" => Some(Token::Colon),
    ";" => Some(Token::SemiColon),
    "+" => Some(Token::Plus),
    "-" => Some(Token::Minus),
    "*" => Some(Token::Asterisk),
    "/" => Some(Token::Slash),
    "%" => Some(Token::Percentage),
    "(" => Some(Token::LParenthesis),
    ")" => Some(Token::RParenthesis),
    "[" => Some(Token::LBracket),
    "]" => Some(Token::RBracket),
    "{" => Some(Token::LBrace),
    "}" => Some(Token::RBrace),
    "<" => Some(Token::Less),
    ">" => Some(Token::Greater),
    "<=" => Some(Token::LessEqual),
    ">=" => Some(Token::GreaterEqual),
    "==" => Some(Token::Equal),
    "!=" => Some(Token::NotEqual),
    "=" => Some(Token::Assigment),
    "@" => Some(Token::At),
    "!" => Some(Token::Not),
    "&&" => Some(Token::And),
    "||" => Some(Token::Or),
    "^" => Some(Token::Xor),
    "?" => Some(Token::Question),
    "->" => Some(Token::Arrow),
    "=>" => Some(Token::DArrow),
    "::" => Some(Token::DColon),
    "'" => Some(Token::SQuote),
    "\"" => Some(Token::DQuote),
    "\"\"\"" => Some(Token::TQuote),

    "as" => Some(Token::As),
    "async" => Some(Token::Async),
    "await" => Some(Token::Await),
    "break" => Some(Token::Break),
    "const" => Some(Token::Const),
    "continue" => Some(Token::Continue),
    "else" => Some(Token::Else),
    "enum" => Some(Token::Enum),
    "false" => Some(Token::False),
    "func" => Some(Token::Func),
    "for" => Some(Token::For),
    "if" => Some(Token::If),
    "impl" => Some(Token::Impl),
    "int" => Some(Token::Int),
    "let" => Some(Token::Let),
    "loop" => Some(Token::Loop),
    "module" => Some(Token::Module),
    "public" => Some(Token::Public),
    "ref" => Some(Token::Ref),
    "return" => Some(Token::Return),
    "static" => Some(Token::Static),
    "struct" => Some(Token::Struct),
    "trait" => Some(Token::Trait),
    "true" => Some(Token::True),
    "type" => Some(Token::Type),
    "unsafe" => Some(Token::Unsafe),
    "use" => Some(Token::Use),
    "var" => Some(Token::Var),
    "where" => Some(Token::Where),
    "while" => Some(Token::While),
    "self" => Some(Token::Self_),

    _ => None,
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

/// Base of numeric literal value. Default numeric base is decimal.
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Base {
  Binary = 2,
  Octal = 8,
  Decimal = 10,
  Hexadecimal = 16,
}
