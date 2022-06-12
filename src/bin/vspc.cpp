#include <stdio.h>
#include <cstdlib>
#include "fwd.h"
#include "cli.h"

#define CMD "vspc"

namespace vsp
{

[[noreturn]]
void
do_print_help_and_exit()
{
  printf(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
%s <source> [ [ --options [ params {...} ] | --options[=params{,...} ] ] {...} ]

where options may any of:

    --fast          This would ignore all other compile options.
    --feature       Enable specified feature.
    --help          Print help message.
    --profile       Activate the specified profile to enable those APIs.
    --version       Print version info.

)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
    , CMD
  );
  std::exit(EXIT_SUCCESS);
}

} // namespace

int
main(int argc, char *argv[])
{
  vsp::fast_return(CMD);
  return 0;
}