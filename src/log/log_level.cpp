#include "fwd.hpp"
#include "log_level.hpp"

namespace vsp
{

namespace log
{

const char* LogLevel::_name[] = {
  "off",
#define LOG_LEVEL(name, text) text,
  LOG_LEVEL_LIST
#undef LOG_LEVEL
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
  return LogLevel::type::Off;
}

};  /*--  namespace vsp::log  --*/

};  /*--  namespace vsp  --*/