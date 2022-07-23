#include "CharBuffer.hpp"

#include "fwd.hpp"

namespace vsp {

namespace fs {

// begin of class vsp::fs::CharBuffer

CharBuffer CharBuffer::allocate(unsigned int capacity) {
#ifdef __linux__
  // char* array = (char*)malloc(capacity * sizeof(char));
  char array[1 << 13] = {0};
#elif _WIN32
#endif
  CharBuffer charBuffer(array);
  return charBuffer;
}

CharBuffer::~CharBuffer() {
#ifdef __linux__
#elif _WIN32
#endif
}

void CharBuffer::wrap(char* array, unsigned int offset, unsigned int length) {}

inline bool CharBuffer::has_array() const { return buff != nullptr; }

CharBuffer CharBuffer::compact() { return *this; }

bool CharBuffer::read_buff(std::ifstream* ifs) {
  if (!ifs->is_open()) {
    return 0;
  }

  unsigned int status = ifs->read(this->buff, 1 << 13).eof();
  this->_limit        = 1 << 13;
  return status;
}

// class vsp::fs::CharBuffer

};  // namespace fs

};  // namespace vsp