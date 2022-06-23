#pragma once
#ifndef _VSP_LOGGING_LOG_LEVEL_H_
#define _VSP_LOGGING_LOG_LEVEL_H_

#define LOG_LEVEL_LIST        \
  LOG_LEVEL(Trace, trace)     \
  LOG_LEVEL(Debug, debug)     \
  LOG_LEVEL(Info, info)       \
  LOG_LEVEL(Warning, warning) \
  LOG_LEVEL(Error, error)
#define LOG_LEVEL_LIST

namespace vsp
{

namespace log
{

class LogLevel
{

public:

enum type {
  Off,
#define LogLevel(name, text) name,
  LOG_LEVEL_LIST
#undef LOG_LEVEL
};

  static const char *name(LogLevel::type level)
  {
    return _name[level];
  }

  static LogLevel::type from_string(const char* str);

private:
  static const char* _name[];

};  /*--  class LogLevel  --*/

typedef LogLevel::type LogLevelType;

};  /*--  namespace vsp::log  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_LOGGING_LOG_LEVEL_H_  --*/