#pragma once
#ifndef _VSP_LOG_LOG_H_
#define _VSP_LOG_LOG_H_

#include "log_level.hpp"

namespace vsp
{

namespace log
{

// #define log_enabled(level, ...) (LogImpl::is_level(LogLevelType))

class LogImpl
{

public:

  LogImpl() {}

  static bool is_enabled(LogLevelType level);

}

};  // namespace log

};  // namespace vsp

#endif  // _VSP_LOG_LOG_H_