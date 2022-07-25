#include "Parser.hpp"

#include "StringUtils.hpp"
#include "fwd.hpp"

namespace vsp {

namespace comp {

// begin of class vps::comp::Parser

// clang-format off
Token Parser::parse_token(const char* lexeme) const {
  Token token = ERROR_TOKEN;
  switch (eval_hash_at_aot(lexeme)) {
#define _TOKEN_M_(name, literal) \
    case eval_hash_at_aot(#literal): \
      std::cout << TokenType::name << ": " << #literal << std::endl; \
      token = Token(TokenType::name, #literal); \
      break;
    _RESERVE_WORD_LIST_M_
#undef _TOKEN_M_
    default:
      std::cout << "Identifier: " << lexeme << std::endl;
  }
  return token;
}
// clang-format on

void Parser::tokenize(char* text) {
  std::list<Token> tokens;
  char             current;
  char*            start = text;
  char*            pos   = start;
  char             forward;
  char*            temp;

  while ((current = *pos) != 0) {
    if (IS_BLANK(current)) {
      forward = *(pos + 1);
      if (forward != 0 && IS_VALID_IDENTIFIER_CHAR(forward)) {
        start = pos + 1;
      }
    } else if (current == '#') {
      while (*pos != 0 && *pos != '\n') {
        pos++;
      }
    } else if (IS_VALID_IDENTIFIER_CHAR(current)) {
      forward = *(pos + 1);
      if (forward != 0 && !IS_VALID_IDENTIFIER_CHAR(forward)) {
#ifdef __linux__
        temp = (char*)malloc((pos - start + 1) * sizeof(char));
        strncpy(temp, start, (pos - start + 1));
        std::cout << temp << std::endl;
        this->parse_token(temp);
        free(temp);
        temp = nullptr;
#endif
      }
    } else if (!IS_VALID_IDENTIFIER_CHAR(current)) {
    } else {
      std::cerr << "Unexpected character: `" << current << "`" << std::endl;
      std::exit(EXIT_FAILURE);
    }
    pos++;
  }
}

// class vps::comp::Parser

};  // namespace comp

};  // namespace vsp