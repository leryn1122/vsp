#include "argparser.hpp"
#include "fwd.hpp"

#define CMD "vspstk"

/**
 *  Entrypoint of Vesperace Stack Trace Tool.
 */
int main(int argc, char *argv[])
{
  auto argparser = vsp::cli::ArgParser(CMD)
      .set_intro("Vesperace Stack Trace Tool")
      .add_help_option()
      .add_version_option();
  return 0;
}