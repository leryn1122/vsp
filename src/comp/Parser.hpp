#pragma once
#ifndef _VSP_COMP_PARSER_H_
#define _VSP_COMP_PARSER_H_

#include "fwd.hpp"

#define DEFAULT_CHAR_BUFFER_SIZE 1<<12

namespace vsp
{

namespace comp
{

/// class CharBuffer
/// 
/// A flipping char buffer to accept the char stream read from the soource file.
/// 
/// ```c++
/// auto buffer = CharBuffer::allocate(1<<10);
/// ```
class CharBuffer
{
public:
  static CharBuffer allocate(unsigned int capacity);

  static void wrap(char* array, unsigned int offset, unsigned int length);

private:
  char* buff;
  unsigned int offset = 0;
  bool read_only = false;

public:
  CharBuffer(){}
  virtual ~CharBuffer();

public:
  
  CharBuffer slice(unsigned int index, unsigned int length);

  CharBuffer duplicate();

  CharBuffer as_readonly_buffer();

  unsigned int capacity();

  char get(unsigned int index);

  char get_unchecked(unsigned int index);

  CharBuffer get_array(unsigned int index, char* array);

  CharBuffer put(char ch);

  inline
  bool has_array() const;

  CharBuffer limit(unsigned int new_limit);

  CharBuffer position(unsigned int new_position);

  CharBuffer clear();

  CharBuffer flip();

  CharBuffer compact();

  char at(unsigned int index);

  bool is_empty() const { return false; }

  unsigned int size();


};  //  class CharBuffer 

class Parser
{

};  //  class vsp::comp::Parser

};  //  namespace vsp::comp

};  //  namespace vsp

#endif  //  _VSP_COMP_PARSER_H_