#pragma once
#ifndef _VSP_LOG_LOG_H_
#define _VSP_LOG_LOG_H_

namespace vsp
{

namespace log
{

#define log_error(...) (!log_is_enaled(Error, ))

class LogImpl
{

public:

  LogImpl() {}

}

};  // namespace log

};  // namespace vsp

#endif  // _VSP_LOG_LOG_H_