#pragma once
#ifndef _VSP_CONTEXT_H_

#include "argparser.hpp"

namespace vsp
{

class Context
{
public:

  static Context initial_from_cli_args(cli::ArgParser args)
  {
    Context context;
    context.args = args;
    return context;
  }

private:
  cli::ArgParser args;

public:

  Context(void) {}
  ~Context() {}

  cli::ArgParser* get_arg_parser()
  {
    return &(this->args);
  }

};  /*--  class Context  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_CONTEXT_H_  --*/