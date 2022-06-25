#include <filesystem>
#include <fstream>
#include <string>

#include "argparser.hpp"
#include "compiler.hpp"
#include "fwd.hpp"

namespace vsp
{

namespace comp
{

/*--  begin of class Compiler  --*/
//class Compiler

  void Compiler::compile(vsp::cli::ArgParser argparser)
  {
    std::string source = argparser.get_argument_str("source");

    std::ifstream ifs(source, std::ios::in);
    if (!ifs.is_open())
    {
      std::cerr << "Error: Cannot open source file `" << source << "`" << std::endl;
      std::exit(EXIT_FAILURE);
    }

    std::string buff;
    while (ifs >> buff)
    {
      std::cout << buff << std::endl;
    }
  }

/*--  class Compiler  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/