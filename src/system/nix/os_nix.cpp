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

};  /*--  namespace vsp::sys  --*/

};  /*--  namespace vsp  --*/