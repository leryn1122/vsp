#include <stdio.h>
#include <cstdlib>

#include "argparser.hpp"
#include "fwd.hpp"

#define CMD "vspx"

int main(int argc, char *argv[])
{
  auto arg_parser = vsp::cli::ArgParser(CMD)
      .set_intro("Vsp Tool of Compression and Decompression")
      .add_help_option()
      .add_version_option();
  return 0;
}