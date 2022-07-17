#include "CharBuffer.hpp"

#include "fwd.hpp"

namespace vsp {

namespace fs {

// begin of class vsp::fs::CharBuffer

CharBuffer CharBuffer::allocate(unsigned int capacity) {
#ifdef __linux__
  char* array = (char*)malloc(capacity * sizeof(char));
#elif _WIN32
  char* array = ;
#endif
  CharBuffer charBuffer(array);
  return charBuffer;
}

CharBuffer::~CharBuffer() {
#ifdef __linux__
  // free(this->buff);
#elif _WIN32
#endif
}

void CharBuffer::wrap(char* array, unsigned int offset, unsigned int length) {}

inline bool CharBuffer::has_array() const { return buff != nullptr; }

CharBuffer CharBuffer::clear() { return *this; }

CharBuffer CharBuffer::compact() { return *this; }

// class vsp::fs::CharBuffer

};  // namespace fs

};  // namespace vsp