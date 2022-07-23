#include "Compiler.hpp"

#include <filesystem>
#include <fstream>

#include "Argparser.hpp"
#include "CharBuffer.hpp"
#include "CompilationContext.hpp"
#include "Log.hpp"
#include "Parser.hpp"
#include "fwd.hpp"

namespace vsp {

namespace comp {

typedef vsp::fs::CharBuffer CharBuffer;

void compile(vsp::cli::ArgParser argparser) {
  Compiler compiler(argparser);
  Context  context;
  compiler.execute(argparser, context);
}

// begin of class vps::comp::Compiler

void Compiler::execute(vsp::cli::ArgParser argparser, Context context) {
  do_prepare();

  do_precompile();

  do_compile();
}

void Compiler::do_prepare() {}

void Compiler::do_precompile() {}

void Compiler::do_compile() {
  this->timer.tik();

  string source = this->argparser.get_argument_str("source");

  std::ifstream ifs(source, std::ios::in);
  if (!ifs.is_open()) {
    std::cerr << "Error: Cannot open source file `" << source << "`"
              << std::endl;
    std::exit(EXIT_FAILURE);
  }

  auto buffer = CharBuffer::allocate(1 << 13);
  while (!buffer.read_buff(&ifs)) {
    buffer.flip();
    buffer.clear();
  }
  ifs.close();
  // std::cout << "read: " << buffer.to_array() << std::endl;

  Parser parser;
  parser.tokenize(buffer.to_array());

  this->timer.tok();
}

};  // namespace comp

};  // namespace vsp