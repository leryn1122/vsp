#pragma once
#ifndef _VSP_COMP_COMPILER_H_
#define _VSP_COMP_COMPILER_H_

#include <filesystem>

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

  void compile()
  {
    std::cout << argparser.get_argument<std::string>("source");
    std::cout << std::filesystem::path::preferred_separator;
  }

};  /*--  class Compiler  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_COMPILER_H_  --*/