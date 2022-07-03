#pragma once
#ifndef _VSP_COMP_SCANNER_H_
#define _VSP_COMP_SCANNER_H_

#include "fwd.hpp"

#include "scanner.hpp"
#include "tokenizer.hpp"

namespace vsp
{

namespace comp
{

/**
 *  Mode for scanner:
 *    - PLAIN for normal;
 *    - DOUBLE_QUOTED for literal string quoted by `"`
 *    - TRIPLE_QUOTED for literal text block quoted by `"""`
 */
enum ScanMode
{
  PLAIN,
  DOUBLE_QUOTED,
  TRIPLE_QUOTED
}

class Scanner
{

private:
  std::vector<Token> _tokens;
  Token _token;
  Token _prev_token;

  ScanMode _scan_mode = ScanMode::PLAIN;

public:

  Scanner(){}
  virtual ~Scanner(){}

}  /*--  class vsp::comp::Scanner  --*/

};  /*--  namespace vsp::comp  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_COMP_SCANNER_H_  --*/