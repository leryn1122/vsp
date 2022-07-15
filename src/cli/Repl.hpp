#pragma once
#ifndef _VSP_CLI_REPL_H_
#define _VSP_CLI_REPL_H_

#include <iostream>

#include "Argparser.hpp"
#include "fwd.hpp"

/**
 *  Create a interactive shell to Vesperace runtime interpreter.
 *
 *
 *
 *
 *
 */
namespace vsp {

namespace cli {

void repl(vsp::cli::ArgParser argparser);

class Shell {
 private:
  string _prompt = "vsp> ";
  int _rownum = 0;
  std::list<std::function<void(void)>> _shutdown_hooks;

 public:
  Shell() {}
  virtual ~Shell() {}

  void run();

  string get_prompt(int rownum);

  string eval(string line);

  [[noreturn]] int terminate();

  void register_shutdown_hook(std::function<void(void)> shutdown_hook);

  void register_builtin_shutdown_hook();

  void store_history();

 private:
};  // class vsp::cli::Shell

};  // namespace cli

};  // namespace vsp

#endif  // _VSP_CLI_REPL_H_