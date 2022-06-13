#include <cstdlib>
#include <stdio.h>
#include "argparser.hpp"
#include "cli.hpp"
#include "fwd.hpp"

#define CMD "vspc"

namespace vsp
{

[[noreturn]]
void
do_print_help_and_exit()
{
#ifdef FORMAT
  std::cout << std::format(" {} ...", CMD) << std::endl;
#else
  printf(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
%s <source> [ --options {...} ]

Options may be in form of

    --options [ params {...} ] or --options[=params{,...} ]

Where options may any of:

    --fast          This would ignore all other compile options.
    --feature       Enable specified feature.
    --help          Print help message.
    --profile       Activate the specified profile to enable those APIs.
    --version       Print version info.

For example:

    vspc source.vsp --profile=prod --feature nightly

)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
#endif
    , CMD
  );
  std::exit(EXIT_SUCCESS);
}

} // namespace vsp

int
main(int argc, char *argv[])
{
  auto arg_parser = vsp::cli::ArgParser(CMD)
      .set_intro("Vsp Language Compiler")
      .add_help_option()
      .add_version_option()
      .add_option("-d", "--debug", "Enable debug mode.")
      .add_option("", "--feature", "Enable specified feature.")
      .add_option("", "--profile", "Activate the specified profile to enable those APIs.")
      .add_option("-v", "--verbose", "Enable verbose mode.")
      .set_example(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
    vspc source.vsp --profile=prod --feature nightly
)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
      );
  arg_parser.parse(argc, argv);
  return 0;
}