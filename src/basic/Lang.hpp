#pragma once
#ifndef _VSP_BASIC_LANG_H_
#define _VSP_BASIC_LANG_H_

#include <cstdarg>
#include <tuple>
#include "fwd.hpp"

namespace vsp
{

#define _NUM_ARGS_M_(...) std::tuple_size<decltype(std::make_tuple(__VA_ARGS__))>::value

namespace basic
{

#ifndef _VSP_BASIC_LANG_exit_with_error_message_
#define _VSP_BASIC_LANG_exit_with_error_message_
void exit_with_error_message(unsigned int argc, ...);
#endif  // _VSP_BASIC_LANG_exit_with_error_message_
#define EXIT_WITH_ERROR_MESSAGE(...) vsp::basic::exit_with_error_message(_NUM_ARGS_M_(__VA_ARGS__), __VA_ARGS__)

};

/// 
template<class E>
class Expected
{
private:
  E exception;
public:

};  //  class vsp::basic::Expected


};  //  namespace vsp

#endif  //  _VSP_BASIC_LANG_H_