#pragma once
#ifndef _VSP_COMP_TOKENIZER_H_
#define _VSP_COMP_TOKENIZER_H_

#include "fwd.hpp"

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

enum Token
{
  EOF,
  ERROR,
#define _TOKEN_M_(name, text) (name,)
  _RESERVE_WORD_LIST_M_
#undef _TOKEN_M_
};  /*--  enum Token::token  --*/

class Tokenizer
{

};  /*--  class Tokenizer  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_TOKENIZER_H_  --*/