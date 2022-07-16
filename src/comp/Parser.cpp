#include "Parser.hpp"

#include "fwd.hpp"

namespace vsp {

namespace comp {

// begin of class vsp::comp::CharBuffer

CharBuffer CharBuffer::allocate(unsigned int capacity) {
#ifdef __linux__
  char* array = (char*)malloc(capacity * sizeof(char));
#elif _WIN32
  char* array = ;
#endif
  CharBuffer charBuffer;
  return charBuffer;
}

CharBuffer::~CharBuffer() {
#ifdef __linux__
  // free(this->buff);
#elif _WIN32
#endif
}

void CharBuffer::wrap(char* array, unsigned int offset, unsigned int length) {}

unsigned int CharBuffer::capacity() const { return 0; }

inline bool CharBuffer::has_array() const { return buff != nullptr; }

CharBuffer CharBuffer::clear() { return *this; }

CharBuffer CharBuffer::compact() { return *this; }

// class vsp::comp::CharBuffer

// begin of class vsp::comp::Parser

// class vsp::comp::Parser

};  // namespace comp

};  // namespace vsp