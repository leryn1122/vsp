#include <stdio.h>
#include <cstdlib>
#include "argparser.hpp"
#include "fwd.hpp"

#define CMD "vsprepl"

namespace vsp
{

} // namespace vsp

int main(int argc, char *argv[])
{
  auto arg_parser = vsp::cli::ArgParser(CMD)
      .set_intro("Vsp REPL (Read-Eval-Print Loop) or Vsp Shell")
      .add_help_option()
      .add_version_option()
      .add_option("-d", "--debug", "Enable debug mode.");
  return 0;
}