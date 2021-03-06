#pragma once
#ifndef _VSP_FWD_H_
#define _VSP_FWD_H_

#include <filesystem>
#include <iostream>
#include <list>
#include <map>
#include <stdexcept>
#include <string>
#include <typeinfo>
#include <vector>

#include "basic/Lang.hpp"
#include "version.hpp"

// Global aliases
typedef std::string string;
namespace fs = std::filesystem;

namespace vsp {

using namespace fs;

using ::std::list;
using ::std::map;
using ::std::vector;

using ::std::exception;
using ::std::int16_t;
using ::std::int32_t;
using ::std::int64_t;
using ::std::int8_t;
using ::std::intmax_t;
using ::std::intptr_t;
using ::std::nullptr_t;
using ::std::ptrdiff_t;
using ::std::size_t;
using ::std::type_info;
using ::std::uint16_t;
using ::std::uint32_t;
using ::std::uint64_t;
using ::std::uint8_t;
using ::std::uintmax_t;
using ::std::uintptr_t;

using ::std::string;

};  // namespace vsp

#endif  // _VSP_FWD_H_