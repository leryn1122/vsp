#include "Argparser.hpp"
#include "Compiler.hpp"
#include "fwd.hpp"

#define CMD "vspc"

/**
 *  Entrypoint of Vesperace Language Compiler.
 *  
 *  
 *  
 */
int main(int argc, char *argv[])
{
  auto argparser = vsp::cli::ArgParser(CMD)
      .set_intro("Vesperace Language Compiler")
      .add_argument("source", "source code file")
      .add_help_option()
      .add_version_option()
      .add_option("", "--debug", "Enable debug mode.")
      .add_option("", "--deprecation", "Locate where deprecated APIs are used.")
      .add_option("", "--depress-warning", "Depress warning info.")
      .add_option("", "--dry-run", "")
      .add_option<string>("", "--feature", "Enable specified feature.", "")
      .add_option<string>("-o", "--output", "Specify the output file.", "")
      .add_option<string>("-p", "--profile", "Activate the specified profile to enable those APIs.", "default")
      .add_option("", "--release", "Specify the release of APIs.")
      .add_option("-s", "--source", "Specify the source file.")
      .add_option("", "--source-path", "Specify the path to source file.")
      .add_option("-t", "--target", "Specify the target file.")
      .add_option("", "--target-path", "Specify the path to target file.")
      .add_option<int>("-T", "--thread", "Set the parallel thread to compile source.", 4)
      .add_option("-V", "--verbose", "Enable verbose mode.")
      .set_example(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
    vspc source.vsp --profile=prod --feature nightly
)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
      )
      .parse(argc, argv);
  vsp::comp::compile(argparser);
  return 0;
}