#include "argparser.hpp"
#include "fwd.hpp"

#define CMD "vspx"

/**
 *  Entrypoint of Vesperace Tool of Compression and Decompression.
 */
int main(int argc, char *argv[])
{
  auto argparser = vsp::cli::ArgParser(CMD)
      .set_intro("Vesperace Tool of Compression and Decompression")
      .add_help_option()
      .add_version_option();
  return 0;
}