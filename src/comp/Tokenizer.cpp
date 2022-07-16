#include "Tokenizer.hpp"

#include "fwd.hpp"

namespace vsp {

namespace comp {

// begin of class vps::comp::Tokenizer

void Tokenizer::parse_token(const char* lexeme) const {
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

// class vps::comp::Tokenizer

};  // namespace comp

};  // namespace vsp