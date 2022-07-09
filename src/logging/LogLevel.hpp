#pragma once
#ifndef _VSP_LOG_LEVEL_H_
#define _VSP_LOG_LEVEL_H_

#define _LOG_LEVEL_LIST_M_        \
  _LOG_LEVEL_M_(TRACE  , trace  ) \
  _LOG_LEVEL_M_(DEBUG  , debug  ) \
  _LOG_LEVEL_M_(INFO   , info   ) \
  _LOG_LEVEL_M_(WARNING, warning) \
  _LOG_LEVEL_M_(ERROR  , error  )

namespace vsp
{

namespace log
{

class LogLevel
{

public:

  enum type {
    OFF,
#define _LOG_LEVEL_M_(name, text) name,
    _LOG_LEVEL_LIST_M_
#undef _LOG_LEVEL_M_
  };  //  enum LogLevel::type

  static const char *name(LogLevel::type level)
  {
    return _name[level];
  }

  static LogLevel::type from_string(const char* str);

private:
  static const char* _name[];

};  //  class vsp::log::LogLevel

typedef LogLevel::type LogLevelType;

};  //  namespace vsp::log

};  //  namespace vsp

#endif  //  _VSP_LOG_LEVEL_H_