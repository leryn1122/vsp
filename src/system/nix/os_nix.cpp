#include <pwd.h>
#include <unistd.h>
#include "fwd.hpp"

namespace vsp
{

namespace sys
{

string get_passwd_name()
{
  struct passwd *pwd = getpwuid(getuid());
  return pwd->pw_name;
}

string get_homedir()
{
  struct passwd *pwd = getpwuid(getuid());
  return string(pwd->pw_dir);
}

string get_tempdir()
{
  return "/tmp";
}

};  //  namespace vsp::sys

};  //  namespace vsp