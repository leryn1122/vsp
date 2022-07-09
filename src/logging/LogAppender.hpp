#pragma once
#ifndef _VSP_LOG_APPENDER_H_
#define _VSP_LOG_APPENDER_H_

#include "fwd.hpp"

namespace vsp
{

namespace log
{

/**
 *  Interface for miscellaneous log appenders.
 *
 */
class LogAppender /* interface */
{
  friend class LogConfiguration;

private:
  bool _reconfigureable = false;


public:

  LogAppender() : _reconfigureable(false){}
  virtual ~LogAppender(){}

};  //  class vsp::log::LogAppender

};  //  namespace vsp::log

};  //  namespace vsp

#endif  //  _VSP_LOG_APPENDER_H_