#pragma once
#ifndef _VSP_COMP_PREPROCESSOR_H_
#define _VSP_COMP_PREPROCESSOR_H_

#include "fwd.hpp"

namespace vsp
{

namespace comp
{

class Preprocessor
{

  Preprocessor(){}
  virtual ~Preprocessor(){}

};  /*--  class vsp::comp::Preprocessor  --*/

class PreprocessorCallback
{

public:

  PreprocessorCallback(){}
  virtual ~PreprocessorCallback(){}

};  /*--  class vsp::comp::PreprocessorCallback  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_PREPROCESSOR_H_  --*/