#pragma once
#ifndef _VSP_COMP_COMPILER_H_
#define _VSP_COMP_COMPILER_H_

#include "Argparser.hpp"
#include "CompilationContext.hpp"
#include "Timer.hpp"
#include "fwd.hpp"

namespace vsp {

namespace comp {

void compile(vsp::cli::ArgParser argparser);

#define _COMPILE_RESULT_LIST_M_ \
  _COMPILE_RESULT_M_(Ok, 0)     \
  _COMPILE_RESULT_M_(Error, 1)

enum CompileResult {
#define _COMPILE_RESULT_M_(name, code) name,
  _COMPILE_RESULT_LIST_M_
#undef _COMPILE_RESULT_M_
};

class Compiler {
 private:
  vsp::cli::ArgParser argparser;
  string name;
  bool verbose;
  std::set<string> _input_files;

  /// Compiler Timer
  vsp::util::Timer timer;

 public:
  Compiler(vsp::cli::ArgParser argparser) : argparser(std::move(argparser)) {
    this->name = this->argparser.has_argument("source");
    this->verbose = this->argparser.has_option("--verbose");
  }
  virtual ~Compiler() {}

  void execute(vsp::cli::ArgParser argparser, Context context);

 private:
  void do_prepare();

  void do_compile();

  void do_precompile();

};  // class Compiler

class CompilationUnit {
 public:
  CompilationUnit() {}
  virtual ~CompilationUnit() {}

};  // vsp::comp::CompilationUnit

};  // namespace comp

};  // namespace vsp

#endif  // _VSP_COMP_COMPILER_H_