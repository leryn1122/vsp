#include "fwd.hpp"
#include "LogLevel.hpp"

namespace vsp
{

namespace log
{

const char* LogLevel::_name[] = {
  "off",
#define _LOG_LEVEL_M_(name, text) #text,
  _LOG_LEVEL_LIST_M_
#undef _LOG_LEVEL_M_
};

LogLevelType LogLevel::from_string(const char* str)
{
  for (uint i = 0; i < 5; i++)
  {
    if (strcmp(str, _name[i]) == 0)
    {
      return static_cast<LogLevelType>(i);
    }
  }
  return LogLevel::type::OFF;
}

};  //  namespace vsp::log

};  //  namespace vsp