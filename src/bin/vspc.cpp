#include "argparser.hpp"
#include "compiler.hpp"
#include "context.hpp"
#include "fwd.hpp"

#define CMD "vspc"

/**
 *  Entrypoint of Vesperace Language Compiler.
 */
int main(int argc, char *argv[])
{
  auto arg_parser = vsp::cli::ArgParser(CMD)
      .set_intro("Vesperace Language Compiler")
      .add_argument("source", "source code file")
      .add_help_option()
      .add_version_option()
      .add_option("", "--debug", "Enable debug mode.")
      .add_option("", "--deprecation", "Locate where deprecated APIs are used.")
      .add_option("", "--depress-warning", "Depress warning info.")
      .add_option("", "--dry-run", "")
      .add_option<std::string>("", "--feature", "Enable specified feature.", "")
      .add_option<std::string>("-o", "--output", "Specify the output file.", "")
      .add_option<std::string>("-p", "--profile", "Activate the specified profile to enable those APIs.", "default")
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
  vsp::Context context = vsp::Context::initial_from_cli_args(&arg_parser);
  vsp::comp::Compiler compiler = vsp::comp::Compiler(context);
  return 0;
}