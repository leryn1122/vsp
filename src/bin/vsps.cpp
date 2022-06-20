#include <stdio.h>
#include <cstdlib>
#include "argparser.hpp"
#include "fwd.hpp"
#include "os_nix.hpp"

#define CMD "vsps"

int main(int argc, char *argv[])
{
  std::cout << vsp::sys::get_passwd_name() << std::endl;
  auto arg_parser = vsp::cli::ArgParser(CMD)
      .set_intro("Vsp Process Tool")
      .add_help_option()
      .add_version_option()
      .parse(argc, argv);
  return 0;
}