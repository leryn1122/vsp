#include "argparser.hpp"
#include "fwd.hpp"
#include "repl.hpp"

namespace vsp
{

namespace cli
{

void repl(vsp::cli::ArgParser argparser)
{
  Shell shell;
  // shell.attach_tty(std::cout);
  shell.run();
}

/*--  begin of class vsp::cli::Shell  --*/
//class Shell

void Shell::attach_tty(std::ostream output)
{
  // this->_output = output;
}

void Shell::run()
{
  std::cout << "Hello, it's Vesperace ~" << std::endl;
  std::cout << this->_prompt;
  string line;
  // while (std::cin >> line && line.size() > 0)
  // char line[8192] = { 0 };
  while(getline(std::cin, line))
  {
    std::cout << "   > " << line << std::endl;
    std::cout << this->_prompt;
  }
}

/*--  class vsp::cli::Shell  --*/

};  /*--  namespace vsp::cli  --*/

};  /*--  namespace vsp  --*/