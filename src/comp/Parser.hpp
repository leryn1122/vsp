#pragma once
#ifndef _VSP_COMP_TOKENIZER_H_
#define _VSP_COMP_TOKENIZER_H_

#include "fwd.hpp"

///
/// - Use precompile macro to generate the enumeration class of token.
///
///
namespace vsp {

namespace comp {

#define IS_BLANK(c) ((c) == '\n' || (c) == '\t' || (c) == ' ')
#define IS_DIGIT(c) ((c) >= '0' && (c) <= '9')
#define IS_ALPHA(c) (((c) >= 'a' && (c) <= 'z') || ((c) >= 'A' && (c) <= 'Z'))
#define IS_ALPHA_OR_DIGIT(c) (((IS_DIGIT(c)) || (IS_ALPHA(c)))
#define IS_VALID_IDENTIFIER_CHAR(c) \
  (((IS_DIGIT(c)) || (IS_ALPHA(c)) || ((c) == '_')))

#define IS_HEX_DIGIT(c) \
  (((c) >= 'A' && (c) <= 'F') || ((c) >= 'a' && (c) <= 'f'))

// clang-format off
#define _RESERVE_WORD_LIST_M_     \
  _TOKEN_M_(AS       , as       ) \
  _TOKEN_M_(ASYNC    , async    ) \
  _TOKEN_M_(AWAIT    , await    ) \
  _TOKEN_M_(BREAK    , break    ) \
  _TOKEN_M_(CHAR     , char     ) \
  _TOKEN_M_(CLASS    , class    ) \
  _TOKEN_M_(CONST    , const    ) \
  _TOKEN_M_(CONTINUE , continue ) \
  _TOKEN_M_(ELSE     , else     ) \
  _TOKEN_M_(ENUM     , enum     ) \
  _TOKEN_M_(FALSE    , false    ) \
  _TOKEN_M_(FUNC     , func     ) \
  _TOKEN_M_(FOR      , for      ) \
  _TOKEN_M_(IF       , if       ) \
  _TOKEN_M_(IMPL     , impl     ) \
  _TOKEN_M_(IMPORT   , import   ) \
  _TOKEN_M_(IN       , in       ) \
  _TOKEN_M_(INT      , int      ) \
  _TOKEN_M_(LET      , let      ) \
  _TOKEN_M_(LOOP     , loop     ) \
  _TOKEN_M_(MODULE   , module   ) \
  _TOKEN_M_(PUBLIC   , public   ) \
  _TOKEN_M_(PRIVATE  , private  ) \
  _TOKEN_M_(REF      , ref      ) \
  _TOKEN_M_(RETURN   , return   ) \
  _TOKEN_M_(STATIC   , static   ) \
  _TOKEN_M_(STRUCT   , struct   ) \
  _TOKEN_M_(SUPER    , super    ) \
  _TOKEN_M_(TRUE     , true     ) \
  _TOKEN_M_(TYPE     , type     ) \
  _TOKEN_M_(UINT     , uint     ) \
  _TOKEN_M_(UNION    , union    ) \
  _TOKEN_M_(UNSAFE   , unsafe   ) \
  _TOKEN_M_(USE      , use      ) \
  _TOKEN_M_(VAR      , var      ) \
  _TOKEN_M_(WHERE    , where    ) \
  _TOKEN_M_(WHILE    , while    )

#define _PUNCTUATION_LIST_M_           \
  _TOKEN_M_(DOT           , "."      ) \
  _TOKEN_M_(COMMA         , ","      ) \
  _TOKEN_M_(SEMI_COLON    , ";"      ) \
  _TOKEN_M_(COLON         , ":"      ) \
  _TOKEN_M_(PLUS          , "+"      ) \
  _TOKEN_M_(MINUS         , "-"      ) \
  _TOKEN_M_(ASTERISK      , "*"      ) \
  _TOKEN_M_(SLASH         , "/"      ) \
  _TOKEN_M_(PERCENTAGE    , "%"      ) \
  _TOKEN_M_(L_PARENTHESIS , "("      ) \
  _TOKEN_M_(R_PARENTHESIS , ")"      ) \
  _TOKEN_M_(L_BRACKET     , "["      ) \
  _TOKEN_M_(R_BRACKET     , "]"      ) \
  _TOKEN_M_(L_BRACE       , "{"      ) \
  _TOKEN_M_(R_BRACE       , "}"      ) \
  _TOKEN_M_(LESS          , "<"      ) \
  _TOKEN_M_(GREATER       , ">"      ) \
  _TOKEN_M_(LESS_EQUAL    , "<="     ) \
  _TOKEN_M_(GREATER_EQUAL , ">="     ) \
  _TOKEN_M_(EQUAL         , "=="     ) \
  _TOKEN_M_(NOT_EQUAL     , "!="     ) \
  _TOKEN_M_(ASSIGMENT     , "="      ) \
  _TOKEN_M_(NOT           , "!"      ) \
  _TOKEN_M_(AND           , "&&"     ) \
  _TOKEN_M_(OR            , "||"     ) \
  _TOKEN_M_(XOR           , "^"      ) \
  _TOKEN_M_(QUESTION      , "?"      ) \
  _TOKEN_M_(S_QUOTE       , "'"      ) \
  _TOKEN_M_(D_QUOTE       , "\""     ) \
  _TOKEN_M_(T_QUOTE       , "\"\"\"" ) \
  _TOKEN_M_(ARROW         , "->"     ) \
  _TOKEN_M_(D_ARROW       , "=>"     ) \
  _TOKEN_M_(D_COLON       , "::"     )

enum TokenType {
  EOF_,
  IDENTIFIER,
  NUMERIC,
  LITERAL,
#define _TOKEN_M_(name, literal) name,
  _RESERVE_WORD_LIST_M_
#undef _TOKEN_M_
#define _TOKEN_M_(name, literal) name,
  _PUNCTUATION_LIST_M_
#undef _TOKEN_M_
  ERROR
};  // enum Token::token
// clang-format on

struct Token {
  Token(TokenType tokenType, std::string attribute)
      : type(std::move(tokenType)), attribute(std::move(attribute)) {}

  TokenType   type;
  std::string attribute;
};

const Token ERROR_TOKEN = Token(ERROR, "");

namespace {

typedef std::uint64_t hash_t;

constexpr hash_t prime = 0x100000001B3ull;
constexpr hash_t basis = 0xCBF29CE484222325ull;

// hash_t hash_(char const* str) {
//   hash_t ret{basis};
//   while (*str) {
//     ret ^= *str;
//     ret *= prime;
//     str++;
//   }
//   return ret;
// }

constexpr hash_t eval_hash_at_aot(char const* str, hash_t last_value = basis) {
  return *str ? eval_hash_at_aot(str + 1, (*str ^ last_value) * prime)
              : last_value;
}

constexpr unsigned long long operator"" _hash(char const* p, size_t) {
  return eval_hash_at_aot(p);
}

};  // namespace

class Parser {
 private:
  std::list<Token> tokens;

 public:
  Token parse_token(const char* lexeme) const;

  void tokenize(char* array);

  std::list<Token> get_tokens() const { return this->tokens; }

};  // class vsp::comp::Parser

};  // namespace comp

};  // namespace vsp

#endif  // _VSP_COMP_TOKENIZER_H_