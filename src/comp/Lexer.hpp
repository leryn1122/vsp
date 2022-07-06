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
};  /*--  struct Position  --*/

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

  const char *_buffer_start;
  const char *_buffer_end;
  const char *_buffer_ptr;

  SourceLocation _source_location;
  ScanMode       _scan_mode;

public:

  void init_internal(const char *buffer_start, const char *buffer_end, const char *buffer_ptr);

  void next_token();

  void prev_token();

};  /*--  class vsp::comp::Lexer  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_LEXER_H_  --*/