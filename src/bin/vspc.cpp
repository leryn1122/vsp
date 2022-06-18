#include "argparser.hpp"
#include "fwd.hpp"

#define CMD "vspc"

namespace vsp
{

};  // namespace vsp

int main(int argc, char *argv[])
{
  auto arg_parser = vsp::cli::ArgParser(CMD)
      .set_intro("Vsp Language Compiler")
      //.add_argument("source", "source code file")
      .add_help_option()
      .add_version_option()
      .add_option("-d", "--debug", "Enable debug mode.")
      .add_option("", "--feature", "Enable specified feature.")
      .add_option("", "--profile", "Activate the specified profile to enable those APIs.")
      .add_option<int>("-t", "--thread", "Set the parallel thread to compile source.", 4)
      .add_option("-v", "--verbose", "Enable verbose mode.")
      .set_example(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
    vspc source.vsp --profile=prod --feature nightly
)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
      )
      .parse(argc, argv);
  return 0;
}