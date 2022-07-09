#pragma once
#ifdef _WIN32
#ifndef _VSP_SYS_OS_NIX_H_
#define _VSP_SYS_OS_NIX_H_

#include "fwd.hpp"

namespace vsp
{

namespace sys
{

string get_passwd_name();

class Nix
{

public:

private:
	static uint32_t _os_version;


};  //  class Nix

};  //  namespace vsp::sys

};  //  namespace vsp

#endif  //  _VSP_SYS_OS_NIX_H_
#endif  //  _WIN32