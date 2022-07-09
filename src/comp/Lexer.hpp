#pragma once
#ifndef _VSP_COMP_LEXER_H_
#define _VSP_COMP_LEXER_H_

#include "fwd.hpp"
#include "SourceLocation.hpp"

namespace vsp
{

namespace comp
{

struct Positiion
{
  Positiion(size_t line, size_t offset)
    : line(std::move(line))
    , offset(std::move(offset))
  {}

  size_t line;
  size_t offset;
};  //  struct Position

const auto NONE  = new Positiion(0,0);
const auto START = new Positiion(1,0);

/// Mode for lexer:
///   - PLAIN for normal;
///   - DOUBLE_QUOTED for literal string quoted by `"`
///   - TRIPLE_QUOTED for literal text block quoted by `"""`
enum ScanMode
{
  PLAIN,
  DOUBLE_QUOTED,
  TRIPLE_QUOTED
};

class Lexer
{

private:

  const char *buffer_start;
  const char *buffer_end;
  const char *buffer_ptr;

  SourceLocation source_location;
  ScanMode       scan_mode;

public:

  void init_internal(const char *buffer_start, const char *buffer_end, const char *buffer_ptr);

  char get_and_advance_char(const char *&ptr);

  const char consume_char(char *ptr, unsigned size);

  char get_char_and_size(const char *ptr, unsigned size);

};  //  class vsp::comp::Lexer

};  //  namespace vsp::comp

};  //  namespace vsp

#endif  //  _VSP_COMP_LEXER_H_