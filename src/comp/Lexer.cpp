#include "fwd.hpp"
#include "Lexer.hpp"
#include "SourceLocation.hpp"

namespace vsp
{

namespace comp
{

/*--  begin of class vsp::comp::Lexer  --*/
//class Lexer

void Lexer::init_internal(const char *buffer_start, const char *buffer_end, const char *buffer_ptr)
{
  _source_location = SourceLocation::START;
  _scan_mode = ScanMode::PLAIN;
}

/*--  class vsp::comp::Lexer  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/