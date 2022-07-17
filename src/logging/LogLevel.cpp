#include "LogLevel.hpp"

#include <cstring>

#include "fwd.hpp"

namespace vsp {

namespace log {

const char* LogLevelType::_name[] = {"off",
#define _LOG_LEVEL_M_(name, text) #text,
                                     _LOG_LEVEL_LIST_M_
#undef _LOG_LEVEL_M_
};

LogLevel LogLevelType::from_string(const char* str) {
  for (uint i = 0; i < 5; i++) {
    if (strcmp(str, _name[i]) == 0) {
      return static_cast<LogLevel>(i);
    }
  }
  return LogLevelType::type::OFF;
}

};  // namespace log

};  // namespace vsp