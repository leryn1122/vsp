#include "CharBuffer.hpp"

#include "fwd.hpp"

namespace vsp {

namespace fs {

// begin of class vsp::fs::CharBuffer

CharBuffer CharBuffer::allocate(unsigned int capacity) {
#ifdef __linux__
  // char  array[capacity * sizeof(char)];
  char* array = (char*)malloc(capacity * sizeof(char));
  // std::cout << capacity * sizeof(char) << std::endl;
#elif _WIN32
#endif
  CharBuffer charBuffer(array);
  // std::cout << sizeof(charBuffer.to_array()) / sizeof(char) << std::endl;
  return charBuffer;
}

CharBuffer::~CharBuffer(){
#ifdef __linux__
// free(this->buff);
#elif _WIN32
#endif
}

CharBuffer CharBuffer::as_readonly_buffer() const {
#ifdef __linux__
  char* buff_s = (char*)malloc(this->cap * sizeof(char));
  strcpy(buff_s, this->buff);
  return buff_s;
#elif _WIN32
  char* array = ;
  return;
#endif
}

void CharBuffer::wrap(char* array, unsigned int offset, unsigned int length) {}

inline bool CharBuffer::has_array() const { return buff != nullptr; }

CharBuffer CharBuffer::compact() { return *this; }

bool CharBuffer::read_buff(std::ifstream* ifs) {
  if (!ifs->is_open()) {
    return 0;
  }

  // std::cout << sizeof(this->buff) / sizeof(char) << std::endl;
  unsigned int status =
      ifs->read(this->buff, sizeof(this->buff) / sizeof(char)).eof();
  this->_limit = sizeof(this->buff) / sizeof(char);
  return status;
}

// class vsp::fs::CharBuffer

};  // namespace fs

};  // namespace vsp