#include <stdio.h>
#include <cstdlib>
#include <string>

#include "version"

namespace vsp
{
namespace cli
{

[[noreturn]]
void
do_print_version_and_exit(char* cmd)
{
  printf(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
%s version %d.%d.%d
)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
   , cmd
   , 0
   , 0
   , 1
   );
  std::exit(EXIT_SUCCESS);
}

void
fast_return_without_primary_args(char *argv[], char* cmd, void(*help_hook)())
{
  int len = 4;
  for (int i = 0; i < len; i++) {
    if ("--help" == argv[i]) {
      help_hook();
    } else if ("--version" == argv[i]) {
      do_print_version_and_exit(cmd);
    }
  }
}

void
fast_return(char* argv[], char* cmd, void(*help_hook)())
{
  fast_return_without_primary_args(argv, cmd, help_hook);
}

} // namespace vsp::cli
} // namespace vsp