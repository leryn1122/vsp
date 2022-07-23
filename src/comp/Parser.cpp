#include "Parser.hpp"

#include "fwd.hpp"

namespace vsp {

namespace comp {

// begin of class vps::comp::Parser

void Parser::parse_token(const char* lexeme) const {
  switch (eval_hash_at_aot(lexeme)) {
    case "public"_hash:
      std::cout << "PUBLIC" << std::endl;
      break;
    case "class"_hash:
      std::cout << "CLASS" << std::endl;
      break;
    case "String"_hash:
      std::cout << "STRING" << std::endl;
      break;
    default:
      std::cout << "Default" << std::endl;
  }
}

void Parser::tokenize(char* array) {
  unsigned int      line;
  char              ch;
  char*             start = array;
  char*             pos   = start;
  std::vector<char> buffer2;

  while ((ch = *pos) != 0) {
    if (ch == '\n') {
      line++;
    } else if (IS_BLANK(ch)) {
      if (!buffer2.empty()) {
        buffer2.clear();
        std::cout << std::endl;
      }
    } else if (ch == '#') {
      while (*pos != 0 && *pos != '\n') {
        pos++;
      }
    } else if (IS_VALID_IDENTIFIER_CHAR(ch)) {
      std::cout << ch;
      buffer2.push_back(ch);
    } else if (!IS_VALID_IDENTIFIER_CHAR(ch)) {
      if (!buffer2.empty()) {
        buffer2.clear();
        std::cout << std::endl;
      }
      std::cout << ch;
      std::cout << std::endl;
    }
    pos++;
  }
}

// class vps::comp::Parser

};  // namespace comp

};  // namespace vsp