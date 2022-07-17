#pragma once
#ifndef _VSP_FS_CHAR_BUFFER_H_
#define _VSP_FS_CHAR_BUFFER_H_

#include <cstring>
#include <fstream>

#include "fwd.hpp"

namespace vsp {

namespace fs {

/// class CharBuffer
///
/// Inspired from `java.nio.CharBuffer`
///
/// A flipping char buffer to accept the char stream read from the source file.
/// The buffer manages set of pointers:
///   - cap   : Capacity of char array;
///   - pos   : Start posistion to read.
///   - limit : Limit of read boundary.
///   - mark  : Marked limit of read boundary.
/// ```c++
/// auto buffer = CharBuffer::allocate(1<<10);
/// ```
///
///
///
class CharBuffer {
 public:
  static CharBuffer allocate(unsigned int capacity);

  static void wrap(char* array, unsigned int offset, unsigned int length);

 private:
  char*        buff;
  unsigned int _mark     = -1;
  unsigned int pos       = 0;
  unsigned int cap       = 0;
  unsigned int _limit    = 0;
  bool         read_only = false;

 public:
  CharBuffer() {}
  CharBuffer(char* buff) : buff(std::move(buff)) {}
  virtual ~CharBuffer();

 public:
  CharBuffer slice(unsigned int index, unsigned int length);

  CharBuffer as_readonly_buffer() const;

  unsigned int capacity() const { return this->cap; }

  char get(unsigned int index) const;

  char get_unchecked(unsigned int index) const;

  CharBuffer get_array(unsigned int index, char* array) const;

  CharBuffer put(char ch);

  inline bool has_array() const;

  CharBuffer limit(unsigned int new_limit) {
    _limit = new_limit;
    if (pos > new_limit) {
      pos = new_limit;
    }
    if (this->_mark > new_limit) {
      this->_mark = -1;
    }
    return *this;
  }

  CharBuffer position(unsigned int new_position) {
    if (this->_mark > new_position) {
      this->_mark = -1;
    }
    this->pos = new_position;
    return *this;
  }

  CharBuffer mark() {
    this->_mark = this->pos;
    return *this;
  }

  CharBuffer reset() {
    this->pos = this->_mark;
    return *this;
  }

  CharBuffer clear() {
    this->pos    = 0;
    this->_limit = this->cap;
    this->_mark  = -1;
    return *this;
  }

  CharBuffer flip() {
    this->_limit = this->pos;
    this->pos    = 0;
    this->_mark  = -1;
    return *this;
  }

  unsigned int remaining() {
    auto remaining = this->_limit - this->pos;
    return remaining > 0 ? remaining : 0;
  }

  const char* to_array() {
    // const char* buff_s = {0};
    // strncpy(buff, buff_s, _limit);
    // return buff_s;
    return this->buff;
  }

  CharBuffer compact();

  char at(unsigned int index) const;

  bool is_empty() const { return false; }

  unsigned int size() const;

  bool read_buff(std::ifstream* ifs);

};  // class vsp::fs::CharBuffer

};  // namespace fs

};  //  namespace vsp

#endif  //  _VSP_FS_CHAR_BUFFER_H_