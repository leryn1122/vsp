#pragma once
#ifndef _VSP_SAMPLE_H_
#define _VSP_SAMPLE_H_

#include <filesystem>
#include "fwd.hpp"

#ifdef __linux__
#include "os_nix.hpp"
#endif

namespace vsp
{

namespace stat
{

struct VMProcessSketch
{
  uint         pid;
  std::string  pname;
};  /*--  struct process  --*/

int get_pid();

std::vector<VMProcessSketch> get_pids()
{
  std::vector<VMProcessSketch> pids;
#ifdef __linux__
  std::string proc_dir;
  proc_dir = proc_dir + "/tmp" + "/" + "vsproc.d" + "/" + vsp::sys::get_passwd_name();
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

#endif  /*--  _VSP_SAMPLE_H_  --*/