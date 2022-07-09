#pragma once
#ifndef _VSP_COMP_TOKENIZER_H_
#define _VSP_COMP_TOKENIZER_H_

#include "fwd.hpp"

/**
 *  - Use precompile macro to generate the enumeration class of token.
 *  
 */
namespace vsp
{

namespace comp
{

#define _RESERVE_WORD_LIST_M_     \
  _TOKEN_M_(AS       , as       ) \
  _TOKEN_M_(ASYNC    , async    ) \
  _TOKEN_M_(AWAIT    , await    ) \
  _TOKEN_M_(BREAK    , break    ) \
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
  _TOKEN_M_(REF      , ref      ) \
  _TOKEN_M_(RETURN   , return   ) \
  _TOKEN_M_(STATIC   , static   ) \
  _TOKEN_M_(STRUCT   , struct   ) \
  _TOKEN_M_(SUPER    , super    ) \
  _TOKEN_M_(TRUE     , true     ) \
  _TOKEN_M_(TYPE     , type     ) \
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
  _TOKEN_M_(D_COLON       , "::"     ) \

enum TokenType
{
  EOF,
  ERROR,
  IDENTIFIER,
  NUMERIC,
  LITERAL,
#define _TOKEN_M_(name, literal) (name,)
  _RESERVE_WORD_LIST_M_
#undef _TOKEN_M_
#define _TOKEN_M_(name, literal) (name,)
  _PUNCTUATION_LIST_M_
#undef _TOKEN_M_
};  //  enum Token::token


struct Token
{
  TokenType   type;
  std::string attribute;
}

class Tokenizer
{

public:

  void parse_token(string lexeme);

};  //  class vsp::comp::Tokenizer

};  //  namespace vsp::comp

};  //  namespace vsp

#endif  //  _VSP_COMP_TOKENIZER_H_