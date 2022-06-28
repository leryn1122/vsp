#include <fstream>

#include "argparser.hpp"
#include "fwd.hpp"
#include "repl.hpp"

#define VSP_HISTORY_FILE ".vsp_history"

#ifdef __linux__
#include "os_nix.hpp"
#endif

namespace vsp
{

namespace cli
{

void repl(vsp::cli::ArgParser argparser)
{
  Shell shell;
  // shell.attach_tty(std::cout);
  shell.register_builtin_shutdown_hook();
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
    std::cout << this->get_prompt(this->_rownum);
    string line;
    while(getline(std::cin, line))
    {
      string result = this->eval(line);
      std::cout << "   > " << result;
      std::cout << std::endl;
      std::cout << this->get_prompt(this->_rownum);
      this->_rownum++;
    }
    terminate();
  }

  string Shell::get_prompt(int rownum) {
    return this->_prompt;
  }

  string Shell::eval(string line)
  {
    if (line == "exit" || line == "quit")
    {
      terminate();
    }
    // TODO: 
    return line;
  }

  [[noreturn]]
  int Shell::terminate()
  {
    for (auto shutdown_hook : this->_shutdown_hooks)
    {
      shutdown_hook();
    }
    std::exit(EXIT_SUCCESS);
  }

  void Shell::register_shutdown_hook(std::function<void(void)> shutdown_hook)
  {
    this->_shutdown_hooks.insert(this->_shutdown_hooks.cend(), shutdown_hook);
  }

  void Shell::register_builtin_shutdown_hook()
  {
    register_shutdown_hook([this](){
      store_history();
    });
    register_shutdown_hook([](){
      std::cout << "\n  Bye ~\n" << std::endl;
    });
  }

  void Shell::store_history()
  {
    std::string history_file = "";
#ifdef __linux__
    history_file = history_file + vsp::sys::get_homedir()
          + fs::path::preferred_separator
          + VSP_HISTORY_FILE;
#elif _WIN32
#endif
    std::ofstream ofs(history_file, std::ios::app|std::ios::out);
    // TODO:
    ofs << "hello" << std::endl;
  }

/*--  class vsp::cli::Shell  --*/

};  /*--  namespace vsp::cli  --*/

};  /*--  namespace vsp  --*/