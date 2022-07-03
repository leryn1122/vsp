#pragma once
#ifndef _VSP_VM_VM_H_
#define _VSP_VM_VM_H_

#include "fwd.hpp"
#include "VmProperty.hpp"

namespace vsp
{

namespace vm
{

struct VirtualMachineStruct
{

};  /*--  VirtualMachineStruct  --*/

class VirtualMachine
{

public:

  static VirtualMachine create_vm();
  static void destory_vm();

  static bool is_supported_vsp_version(int version);


private:


  std::vector<SystemProperty> _system_properites;

public:

  VirtualMachine(){}
  virtual ~VirtualMachine(){}

private:
  void init_specific_system_properites();


};  /*--  class vsp::vm::VirtualMachine  --*/

};  /*--  namespace vsp::vm  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_VM_VM_H_  --*/