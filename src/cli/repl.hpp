#pragma once
#ifndef _VSP_CLI_REPL_H_
#define _VSP_CLI_REPL_H_

#include <iostream>

#include "argparser.hpp"
#include "fwd.hpp"

namespace vsp
{

namespace cli
{

void repl(vsp::cli::ArgParser argparser);

class Shell
{

private:
  std::string   _prompt = "vsp> ";

public:
  
  Shell(){}
  virtual ~Shell(){}

  void attach_tty(std::ostream output);

  void run();

};  /*--  class cli  --*/

};  /*--  namespace vsp::cli  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_CLI_REPL_H_  --*/