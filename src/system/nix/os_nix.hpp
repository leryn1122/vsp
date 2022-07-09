#pragma once
#ifdef __linux__
#ifndef _VSP_SYS_OS_NIX_H_
#define _VSP_SYS_OS_NIX_H_

#include "fwd.hpp"

namespace vsp
{

namespace sys
{

string get_passwd_name();

string get_homedir();

string get_tempdir();

class Nix
{

public:

private:
  static uint32_t _os_version;


};  //  class Nix

};  //  namespace vsp::sys

};  //  namespace vsp

#endif  //  _VSP_SYS_OS_NIX_H_
#endif  //  __linux__