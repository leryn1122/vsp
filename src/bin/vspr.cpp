#include <stdio.h>
#include <cstdlib>
#include "fwd.h"
#include "cli.h"

#define CMD "vspr"

namespace vsp
{

[[noreturn]]
void
do_print_help_and_exit()
{
  printf(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
%s

where options may any of:

    --help          Print help message.
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
  return 0;
}