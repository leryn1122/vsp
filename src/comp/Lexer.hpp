#pragma once
#ifndef _VSP_COMP_LEXER_H_
#define _VSP_COMP_LEXER_H_

#include "fwd.hpp"

namespace vsp
{

namespace comp
{

struct Positiion
{
  Positiion(uint line, uint offset)
    : line(std::move(line))
    , offset(std::move(offset))
  {}

  uint line;
  uint offset;
};  /*--  struct Position  --*/

const auto NONE = new Positiion(0,0);
const auto START = new Positiion(1,0);

class Lexer
{

public:

  void next_token();

  void prev_token();

  void split();

};  /*--  class vsp::comp::Lexer  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_LEXER_H_  --*/