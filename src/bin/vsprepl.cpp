#include "Argparser.hpp"
#include "fwd.hpp"
#include "Repl.hpp"

#define CMD "vsprepl"

/**
 *  Entrypoint of Vesperace REPL (Read-Eval-Print Loop) or Vesperace Shell.
 *  
 *  Like `jshell` in Java, `node` in Node.js, `vsprepl` is an interactive
 *  shell / command line interface for Vesperace.
 *  
 */
int main(int argc, char *argv[])
{
  auto argparser = vsp::cli::ArgParser(CMD)
      .set_intro("Vesperace REPL (Read-Eval-Print Loop) or Vesperace Shell")
      .add_help_option()
      .add_version_option()
      .add_option("-d", "--debug", "Enable debug mode.")
      .add_option<string>("", "--prompt", "Customize CLI prompt.", "vsp> ")
      .parse(argc, argv);
  vsp::cli::repl(argparser);
  return 0;
}