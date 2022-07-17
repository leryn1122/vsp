#pragma once
#ifndef _VSP_FS_CHAR_BUFFER_H_
#define _VSP_FS_CHAR_BUFFER_H_

#include "fwd.hpp"

namespace vsp {

namespace fs {

/// class CharBuffer
///
/// A flipping char buffer to accept the char stream read from the soource file.
///
/// ```c++
/// auto buffer = CharBuffer::allocate(1<<10);
/// ```
class CharBuffer {
 public:
  static CharBuffer allocate(unsigned int capacity);

  static void wrap(char* array, unsigned int offset, unsigned int length);

 private:
  char*        buff;
  unsigned int mark      = -1;
  unsigned int _position = 0;
  unsigned int _capacity = 0;
  unsigned int _limit    = 0;
  unsigned int offset    = 0;
  bool         read_only = false;

 public:
  CharBuffer() {}
  CharBuffer(char* buff) : buff(std::move(buff)) {}
  virtual ~CharBuffer();

 public:
  CharBuffer slice(unsigned int index, unsigned int length);

  CharBuffer duplicate();

  CharBuffer as_readonly_buffer() const;

  unsigned int capacity() const { return this->_capacity; }

  char get(unsigned int index) const;

  char get_unchecked(unsigned int index) const;

  CharBuffer get_array(unsigned int index, char* array) const;

  CharBuffer put(char ch);

  inline bool has_array() const;

  CharBuffer limit(unsigned int new_limit);

  CharBuffer position(unsigned int new_position) {
    if (this->mark > new_position) {
      this->mark = -1;
    }
    this->_position = new_position;
    return *this;
  }

  CharBuffer clear();

  CharBuffer flip();

  CharBuffer compact();

  char at(unsigned int index) const;

  bool is_empty() const { return false; }

  unsigned int size() const;

  bool read_buff(std::istream&) { return true; }

};  // class vsp::fs::CharBuffer

};  // namespace fs

};  //  namespace vsp

#endif  //  _VSP_FS_CHAR_BUFFER_H_