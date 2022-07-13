#pragma once
#ifndef _VSP_COMP_CTX_H_
#define _VSP_COMP_CTX_H_

#include "LogConfiguration.hpp"
#include "fwd.hpp"

namespace vsp
{

namespace comp
{

class Context
{
private:
  vsp::log::LogConfiguration logConfiguration;

public:
  Context(){}
  virtual ~Context(){}

private:

  

};  //  class vsp::comp::Context

};  //  namespace vsp::comp

};  //  namespace vsp

#endif  //  _VSP_COMP_CTX_H_