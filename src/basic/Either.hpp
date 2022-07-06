#pragma once
#ifndef _VSP_BASIC_EITHER_H_
#define _VSP_BASIC_EITHER_H_

#include "fwd.hpp"

namespace vsp
{

namespace basic
{

template<typename T>
class Either
{

private:
  std::exception left;
  T  right;

};  /*--  class vsp::basic::Either  --*/

};  /*--  namespace vsp::basic  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_BASIC_EITHER_H_  --*/