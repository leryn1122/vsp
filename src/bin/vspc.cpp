#include <cstdlib>
#include <stdio.h>
#include "argparse.hpp"
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

} // namespace

int
main(int argc, char *argv[])
{
//  vsp::cli::fast_return(argc, argv, CMD, vsp::do_print_help_and_exit);
  auto args = vsp::cli::ArgParser(CMD)
      .set_description("`vspc` for `vsp` language compiler.")
      .add_help_option()
      .add_version_option()
      .add_option("", "--verbose", "123", "");
  return 0;
}