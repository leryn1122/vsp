#pragma once
#ifndef _VSP_COMP_COMPILER_H_
#define _VSP_COMP_COMPILER_H_

#include "context.hpp"

namespace vsp
{

namespace comp
{

class Compiler
{
private:
  vsp::Context context;

public:
  Compiler(vsp::Context context) : context(std::move(context)) {}
  virtual ~Compiler(){}

};  /*--  class Compiler  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_COMPILER_H_  --*/