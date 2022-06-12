#include <stdio.h>
#include <cstdlib>
#include <cstring>

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
R"(%s version %d.%d.%d)"
//==============================================================================
   , cmd
   , 0
   , 0
   , 1
  );
  std::exit(EXIT_SUCCESS);
}

void
fast_return_without_primary_args(int argc, char *argv[], char* cmd, void(*help_hook)())
{
  for (int i = 0; i < argc; i++)
  {
    if (std::strcmp("--help", argv[i]) == 0)
    {
      help_hook();
    }
    else if (std::strcmp("--version", argv[i]) == 0)
    {
      do_print_version_and_exit(cmd);
    }
  }
}

void
fast_return(int argc, char* argv[], char* cmd, void(*help_hook)())
{
  if (argc < 2) { help_hook(); } // Accept at least 2 args or fails.
  fast_return_without_primary_args(argc, argv, cmd, help_hook);
}

}; // namespace vsp::cli
}; // namespace vsp