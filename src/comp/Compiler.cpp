#include <filesystem>
#include <fstream>

#include "Argparser.hpp"
#include "CompilationContext.hpp"
#include "Compiler.hpp"
#include "fwd.hpp"
#include "Log.hpp"

namespace vsp
{

namespace comp
{

#define IS_BLANK(c) ((c) == ' ' || (c) == '\t')
#define IS_DIGIT(c) ((c) >= '0' && (c) <= '9')
#define IS_ALPHA(c) ( ((c) >= 'a' && (c) <= 'z') || ((c) >= 'A' && (c) <= 'Z') )
#define IS_ALPHADIGIT(c) ( ((c) >= 'a' && (c) <= 'z') || ((c) >= 'A' && (c) <= 'Z') || ((c) >= '0' && (c) <= '9') )
#define IS_HEX_DIGIT(c) (((c) >= 'A' && (c) <= 'F') || ((c) >= 'a' && (c) <= 'f'))

void compile(vsp::cli::ArgParser argparser)
{
  Compiler compiler(argparser);
  Context context; 
  compiler.execute(argparser, context);
}

//  begin of class vps::comp::Compiler
//class Compiler {

  void Compiler::execute(vsp::cli::ArgParser argparser, Context context)
  {
    do_prepare();

    do_precompile();

    do_compile();

  }

  void Compiler::do_prepare()
  {
    log_info("Do preparation for compilation.");
  }

  void Compiler::do_precompile()
  {
    log_info("Do precompile.");
  }

  void Compiler::do_compile()
  {
    this->_timer.tik();

    log_info("Do compilation.");

    string source = this->_argparser.get_argument_str("source");

    std::ifstream ifs(source, std::ios::in);
    if (!ifs.is_open())
    {
      std::cerr << "Error: Cannot open source file `" << source << "`" << std::endl;
      std::exit(EXIT_FAILURE);
    }

    string buff;
    while (getline(ifs, buff))
    {
      // for (char c : buff)
      for (size_t i = 0, j = 0; i < buff.size(); i++)
      {
        char ch = buff.at(i);
        if (IS_ALPHADIGIT(ch))
        {
          
        }
      }
    }
    ifs.close();

    this->_timer.tok();
  }


//  class vps::comp::Compiler

};  //  namespace vsp::comp

};  //  namespace vsp