#pragma once
#ifndef _VSP_STAT_PROCESS_H_
#define _VSP_STAT_PROCESS_H_

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

int get_pid();

std::vector<VMProcessSketch> get_pids();

};  /*--  namespace vsp::stat  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_STAT_PROCESS_H_  --*/