#pragma once
#ifndef _VSP_GLOBAL_H_
#define _VSP_GLOBAL_H_

#include <cstring>
#include <stdexcept>
#include <string>
#include <typeinfo>
#include <cstddef>
#include <cstdint>

#include "version.hpp"

namespace vsp {

using ::std::nullptr_t;
using ::std::max_align_t;
using ::std::int8_t;
using ::std::uint8_t;
using ::std::int16_t;
using ::std::uint16_t;
using ::std::int32_t;
using ::std::uint32_t;
using ::std::int64_t;
using ::std::uint64_t;
using ::std::intptr_t;
using ::std::uintptr_t;
using ::std::intmax_t;
using ::std::uintmax_t;
using ::std::ptrdiff_t;
using ::std::size_t;
using ::std::exception;
using ::std::type_info;

using ::std::string;


}; // namespace vsp

#endif // _VSP_GLOBAL_H_