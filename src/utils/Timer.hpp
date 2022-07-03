#pragma once
#ifndef _VSP_UTILS_TIMER_H_
#define _VSP_UTILS_TIMER_H_

#include <chrono>

namespace vsp
{

namespace util
{

class Timer
{

public:

  Timer(){}
  virtual ~Timer(){}

  void tik(){}
  void tok(){}

};  /*--  class vsp::util::Timer  --*/

};  /*--  namespace vsp::util  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_UTILS_TIMER_H_  --*/