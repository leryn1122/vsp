#pragma once
#ifndef _VSP_LOG_LEVEL_H_
#define _VSP_LOG_LEVEL_H_

#define _LOG_LEVEL_LIST_M_        \
  _LOG_LEVEL_M_(Trace  , trace  ) \
  _LOG_LEVEL_M_(Debug  , debug  ) \
  _LOG_LEVEL_M_(Info   , info   ) \
  _LOG_LEVEL_M_(Warning, warning) \
  _LOG_LEVEL_M_(Error  , error  )

namespace vsp
{

namespace log
{

class LogLevel
{

public:

  enum type {
    Off,
#define _LOG_LEVEL_M_(name, text) name,
    _LOG_LEVEL_LIST_M_
#undef _LOG_LEVEL_M_
  };  /*--  enum LogLevel::type  --*/

  static const char *name(LogLevel::type level)
  {
    return _name[level];
  }

  static LogLevel::type from_string(const char* str);

private:
  static const char* _name[];

};  /*--  class vsp::log::LogLevel  --*/

typedef LogLevel::type LogLevelType;

};  /*--  namespace vsp::log  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_LOG_LEVEL_H_  --*/