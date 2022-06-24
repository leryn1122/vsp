#pragma once
#ifndef _VSP_STAT_PROCESS_H_
#define _VSP_STAT_PROCESS_H_

#ifdef __linux__
#include "os_nix.hpp"
#include "vm_constant.hpp"
#endif

namespace vsp
{

namespace stat
{

struct VMProcessSketch
{
  VMProcessSketch(uint pid, std::string pname)
    : pid(std::move(pid))
    , pname(std::move(pname))
  {}
  uint         pid;
  std::string  pname;
};  /*--  struct process  --*/

std::vector<VMProcessSketch> get_pids()
{
  std::vector<VMProcessSketch> pids;
#ifdef __linux__
  std::string proc_dir;
  // Path: /tmp/vsproc.d/<username>
  proc_dir = proc_dir + sys::get_tempdir()
           + fs::path::preferred_separator
           + VM_PROC_DIR
           + fs::path::preferred_separator
           + sys::get_passwd_name();

  fs::path proc_dir_path = proc_dir;  
  if (!fs::exists(proc_dir_path))
  {
    return pids; 
  }

  for (auto& dir_entry: fs::directory_iterator(proc_dir))
  {
    int pid = std::atoi(dir_entry.path().filename().c_str());
    // TODO: read the program name later     
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

#endif  /*--  _VSP_STAT_PROCESS_H_  --*/