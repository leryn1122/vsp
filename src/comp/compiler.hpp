#pragma once
#ifndef _VSP_COMP_COMPILER_H_
#define _VSP_COMP_COMPILER_H_

#include "argparser.hpp"
#include "comp_ctx.hpp"
#include "fwd.hpp"

namespace vsp
{

namespace comp
{

void compile(vsp::cli::ArgParser argparser);

#define _COMPILE_RESULT_LIST_M_ \
  _COMPILE_RESULT_M_(Ok, 0)     \
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

  void execute(vsp::cli::ArgParser argparser, Context context);

private:

  void do_prepare();

  void do_compile();

  void do_precompile();

};  /*--  class Compiler  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_COMPILER_H_  --*/