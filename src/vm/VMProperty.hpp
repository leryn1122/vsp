#pragma once
#ifndef _VSP_VM_PROPERTY_H_
#define _VSP_VM_PROPERTY_H_

#include "fwd.hpp"

namespace vsp {

namespace vm {

class SystemProperty {
 private:
  char* _key;
  char* _value;
  bool _read_only;

 public:
  const char* key() const { return this->_key; }
  char* value() const { return this->_value; }
  bool is_read_only() const { return this->_read_only; }

  SystemProperty(char* key, char* value, bool read_only = false) {
    _key = key;
    _value = value;
    _read_only = read_only;
  }

};  // class vsp::vm::SystemProperty

};  // namespace vm

};  // namespace vsp

#endif  // _VSP_VM_PROPERTY_H_