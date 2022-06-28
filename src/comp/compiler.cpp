#include <filesystem>
#include <fstream>

#include "argparser.hpp"
#include "compiler.hpp"
#include "comp_ctx.hpp"
#include "fwd.hpp"

namespace vsp
{

namespace comp
{

void compile(vsp::cli::ArgParser argparser)
{
  Compiler compiler(argparser);
  Context context; 
  compiler.execute(argparser, context);
}

/*--  begin of class vps::comp::Compiler  --*/
//class Compiler {

  void Compiler::execute(vsp::cli::ArgParser argparser, Context context)
  {

    do_prepare();

    do_precompile();

    do_compile();

  }

  void Compiler::do_prepare()
  {
    std::cout << "Do preparation for compilation." << std::endl;
  }

  void Compiler::do_precompile()
  {
    std::cout << "Do precompile." << std::endl;
  }

  void Compiler::do_compile()
  {
    std::cout << "Do compilation." << std::endl;

    string source = this->_argparser.get_argument_str("source");

    std::ifstream ifs(source, std::ios::in);
    if (!ifs.is_open())
    {
      std::cerr << "Error: Cannot open source file `" << source << "`" << std::endl;
      std::exit(EXIT_FAILURE);
    }

    string buff;
    while (ifs >> buff)
    {
      std::cout << buff << std::endl;
    }
    ifs.close();
  
  }


/*--  class vps::comp::Compiler  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/