#include <stdio.h>
#include <cstdlib>
#include <string>

namespace vsp
{

using std::string;

void
fast_return(char* cmd)
{
  printf("Fast return: %s", cmd);
}


void
fast_return_without_primary_args(char *argv[], char* cmd)
{
}


[[noreturn]]
void
do_print_version_and_exit(char* cmd)
{
  printf(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
%s version %s (early access)
)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
   , cmd
   , "1.0.0"
   );
  std::exit(EXIT_SUCCESS);
}



} // namespace vsp