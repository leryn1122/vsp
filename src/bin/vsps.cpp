#include "Argparser.hpp"
#include "Process.hpp"
#include "fwd.hpp"

#define CMD "vsps"

///
/// Entrypoint of Vesperace Process Status Tool.
///
/// The instinct is very simple inspired by JVM.
/// When a runtime program ran through `vspr`, it created a file named with
/// process ID with content of process. The default location is a directory
/// named `vsproc.d` appending the current username under the system temporary
/// directory:
///
/// ```
/// /tmp/vsproc.d/<username>    (Linux)
/// ```
///
namespace vsp {

namespace cli {

void execute(cli::ArgParser *argparser) {
  bool quiet = argparser->has_option("--quiet");
  std::vector<vsp::stat::VMProcessSketch> processes = vsp::stat::get_pids();
  for (auto &process : processes) {
    std::cout << process.pid;
    if (!quiet) {
      std::cout << " " << process.pname;
    }
    std::cout << std::endl;
  }
}

};  // namespace cli

};  // namespace vsp

int main(int argc, char *argv[]) {
  auto argparser =
      vsp::cli::ArgParser(CMD)
          .set_intro("Vesperace Process Status Tool")
          // .add_argument("PID", "process id")
          .add_help_option()
          .add_version_option()
          .add_option("-q", "--quiet", "Print PIDs only without other info.")
          .set_example(R"(
    vsps | grep app   (Linux) 
    vsps -q
)")
          .parse(argc, argv);
  vsp::cli::execute(&argparser);
  return 0;
}
