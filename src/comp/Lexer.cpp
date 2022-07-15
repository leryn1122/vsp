#include "Lexer.hpp"

#include "SourceLocation.hpp"
#include "fwd.hpp"

namespace vsp {

namespace comp {

// begin of class vsp::comp::Lexer
// class Lexer

void Lexer::init_internal(const char *buffer_start, const char *buffer_end,
                          const char *buffer_ptr) {
  // _source_location = SourceLocation::START;
  this->scan_mode = ScanMode::PLAIN;
}

inline char Lexer::get_and_advance_char(const char *&ptr) { return '0'; }

const char Lexer::consume_char(char *ptr, unsigned size) { return '0'; }

inline char get_char_and_size(const char *ptr, unsigned size) { return '0'; }

// class vsp::comp::Lexer

};  // namespace comp

};  // namespace vsp