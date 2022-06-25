#include <filesystem>
#include <fstream>
#include <string>

#include "argparser.hpp"
#include "fwd.hpp"

namespace vsp
{

namespace comp
{

class Compiler
{
private:
  vsp::cli::ArgParser argparser;

public:

  Compiler(vsp::cli::ArgParser argparser): argparser(std::move(argparser)){}
  virtual ~Compiler(){}

  void compile()
  {
    std::string source = argparser.get_argument_str("source");

    std::ifstream ifs(source, std::ios::in);
    if (!ifs.is_open())
    {
      std::cerr << "Error: Cannot open source file `" << source << "`" << std::endl;
      std::exit(EXIT_FAILURE);
    }
    
    std::string buff;
    if (ifs >> buff)
    {
      std::cout << buff << std::endl;
    }
  }

};  /*--  class Compiler  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/