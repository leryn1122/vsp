#pragma once
#ifndef _VSP_LOGGING_LOG_H_
#define _VSP_LOGGING_LOG_H_

#include <cstdarg>
#include <tuple>

#include "LogLevel.hpp"
#include "fwd.hpp"

namespace vsp {

namespace log {

///
/// Logging macros
///
/// ```
/// #include "Log.hpp"
///
/// log_<level>("This is a ", "sample", " log.");
/// ```
#define log_trace(...) _LOG_WRITE_M_(vsp::log::LogLevel::TRACE, __VA_ARGS__)
#define log_debug(...) _LOG_WRITE_M_(vsp::log::LogLevel::DEBUG, __VA_ARGS__)
#define log_info(...) _LOG_WRITE_M_(vsp::log::LogLevel::INFO, __VA_ARGS__)
#define log_warning(...) _LOG_WRITE_M_(vsp::log::LogLevel::WARNING, __VA_ARGS__)
#define log_error(...) _LOG_WRITE_M_(vsp::log::LogLevel::ERROR, __VA_ARGS__)

#define _LOG_ENABLED_M_(level) (vsp::log::LogImpl::is_enabled(level))
#define _LOG_WRITE_M_(level, ...) \
  !_LOG_ENABLED_M_(level)         \
      ? (void)0                   \
      : vsp::log::LogImpl::write(_NUM_ARGS_M_(__VA_ARGS__), __VA_ARGS__)

class LogImpl {
 private:
  LogLevel level = LogLevel::INFO;

 public:
  LogImpl() {}
  virtual ~LogImpl() {}

  static bool is_enabled(LogLevel level) { return true; }

  static void write(unsigned int argc, ...) {
    va_list args_ptr;
    va_start(args_ptr, argc);
    for (unsigned int i = 0; i < argc; i++) {
      std::cout << va_arg(args_ptr, const char*);
    }
    std::cout << std::endl;
    va_end(args_ptr);
  }

};  // class vsp::log::LogImpl

};  // namespace log

};  // namespace vsp

#endif  // _VSP_LOGGING_LOG_H_