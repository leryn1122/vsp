#include <pwd.h>
#include <unistd.h>
#include "fwd.hpp"

namespace vsp
{

namespace sys
{

std::string get_passwd_name()
{
  struct passwd *pwd = getpwuid(getuid());
  return pwd->pw_name;
}

std::string get_tempdir()
{
  return "/tmp";
}

};  /*--  namespace vsp::sys  --*/

};  /*--  namespace vsp  --*/