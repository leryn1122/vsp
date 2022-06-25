#pragma once
#ifndef _VSP_COMP_COMPILER_H_
#define _VSP_COMP_COMPILER_H_

#include "argparser.hpp"
#include "fwd.hpp"

namespace vsp
{

namespace comp
{

#define _COMPILE_RESULT_LIST_M_ \
  _COMPILE_RESULT_M_(Ok, 0) \
  _COMPILE_RESULT_M_(Error, 1)

enum CompileResult
{
#define _COMPILE_RESULT_M_(name, code) name,
  _COMPILE_RESULT_LIST_M_
#undef _COMPILE_RESULT_M_
};

class Compiler
{

private:
  vsp::cli::ArgParser _argparser;

public:

  Compiler(vsp::cli::ArgParser argparser): _argparser(std::move(argparser)){}
  virtual ~Compiler(){}

  void compile(vsp::cli::ArgParser argparser);

};  /*--  class Compiler  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_COMPILER_H_  --*/