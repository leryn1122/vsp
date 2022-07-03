#pragma once
#ifndef _VSP_STAT_PROCESS_H_
#define _VSP_STAT_PROCESS_H_

#ifdef __linux__
#include "os_nix.hpp"
#include "VmConstant.hpp"
#endif

namespace vsp
{

namespace stat
{

struct VMProcessSketch
{
  VMProcessSketch(uint pid, string pname)
    : pid(std::move(pid))
    , pname(std::move(pname))
  {}
  uint         pid;
  string  pname;
};  /*--  struct process  --*/

std::vector<VMProcessSketch> get_pids();

};  /*--  namespace vsp::stat  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_STAT_PROCESS_H_  --*/