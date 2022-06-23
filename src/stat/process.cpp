#include <filesystem>
#include "fwd.hpp"
#include "process.hpp"

#ifdef __linux__
#include "os_nix.hpp"
#include "vm_constant.hpp"
#endif

namespace vsp
{

namespace stat
{

int get_pid()
{
  return 0;
}

std::vector<VMProcessSketch> get_pids()
{
  std::vector<VMProcessSketch> pids;
#ifdef __linux__
  std::string proc_dir;
  // Path: /tmp/vsproc.d/<username>
  proc_dir = proc_dir + sys::get_tempdir()
           + std::filesystem::path::preferred_separator
           + VM_PROC_DIR
           + std::filesystem::path::preferred_separator
           + sys::get_passwd_name();
  for (auto& dir_entry: std::filesystem::directory_iterator(proc_dir))
  {
    int pid = std::atoi(dir_entry.path().filename().c_str());
    std::string pname = "vsp";
    // std::cout << std::atoi(dir_entry.path().filename().c_str()) << '\n';
    pids.emplace_back(pid, pname);
  }
#elif _WIN32
#endif
  return pids;
}

};  /*--  namespace vsp::stat  --*/

};  /*--  namespace vsp  --*/