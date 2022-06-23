#include <stdio.h>
#include <cstdlib>
#include "argparser.hpp"
#include "fwd.hpp"

#define CMD "vspr"

/**
 *  Entrypoint of Vesperace Runtime Launcher.
 */
int main(int argc, char *argv[])
{
  auto argparser = vsp::cli::ArgParser(CMD)
      .set_intro("Vesperace Runtime Launcher")
      .add_help_option()
      .add_version_option()
      .add_option("-d", "--daemon", "Run the application in daemon mode.")
      .add_option("", "--debug", "Enable debug mode.")
      .add_option("", "--feature", "Enable specified feature.")
      .add_option("", "--profile", "Activate the specified profile to enable those APIs.")
      .add_option("-v", "--verbose", "Enable verbose mode.")
      .set_example(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
    vspr source.vspx
)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
      );
  return 0;
}