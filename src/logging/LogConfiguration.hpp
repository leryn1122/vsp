#pragma once
#ifndef _VSP_LOG_CONF_H_
#define _VSP_LOG_CONF_H_

#include "LogAppender.hpp"

namespace vsp {

namespace log {

/**
 *  Log configuration: Singleton class.
 *
 */
class LogConfiguration {
 private:
  static LogAppender** _appenders;

 public:
  LogConfiguration() {}
  virtual ~LogConfiguration() {}

};  // class vsp::log::LogConfiguration

};  // namespace log

};  // namespace vsp

#endif  // _VSP_LOG_CONF_H_