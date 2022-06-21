#include "argparser.hpp"
#include "context.hpp"
#include "fwd.hpp"
#include "process.hpp"

#define CMD "vsps"

/**
 *  Entrypoint of vsp process tool.
 *  
 *  The instinct is very simple inspired by JVM.
 *  When a runtime program ran through `vspr`, it created a file named with process
 *  ID with content of process.
 *  The default location is a directory named `vsproc.d` appending the current
 *  username under the system temporary directory:
 *  
 *  ```
 *  /tmp/vsproc.d/<username>    (Linux)
 *  ```
 */
namespace vsp
{

namespace cli
{

void execute(cli::ArgParser* arg_parser)
{
  bool quiet = arg_parser->has_option("--quiet");
  std::vector<vsp::stat::VMProcessSketch> processes = vsp::stat::get_pids();
  for (auto &process : processes)
  {
    std::cout << process.pid;
    if (!quiet)
    {
      std::cout << " " << process.pname;
    }
    std::cout << std::endl;
  }
}

};  /*--  namespace vsp::cli  --*/

};  /*--  namespace vsp  --*/


int main(int argc, char *argv[])
{
  auto arg_parser = vsp::cli::ArgParser(CMD)
      .set_intro("Vsp Process Tool")
      // .add_argument("PID", "process id")
      .add_help_option()
      .add_version_option()
      .add_option("-q", "--quiet", "Print PIDs only without other info.")
      .set_example(
//==============================================================================
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""" R"(
    vsps -q
)" """"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
//==============================================================================
      )
      .parse(argc, argv);
  vsp::cli::execute(&arg_parser);
  return 0;
}
