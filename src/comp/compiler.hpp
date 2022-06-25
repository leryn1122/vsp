#pragma once
#ifndef _VSP_COMP_COMPILER_H_
#define _VSP_COMP_COMPILER_H_

#include "fwd.hpp"

namespace vsp
{

namespace comp
{

class Compiler
{
private:
  vsp::cli::ArgParser argparser;

public:

  Compiler(vsp::cli::ArgParser argparser): argparser(std::move(argparser)){}
  virtual ~Compiler(){}

  void compile(void);

};  /*--  class Compiler  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_COMPILER_H_  --*/