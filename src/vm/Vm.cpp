#include "fwd.hpp"
#include "Vm.hpp"
#include "VmProperty.hpp"

namespace vsp
{

namespace vm
{



VirtualMachine VirtualMachine::create_vm()
{
  if (!is_supported_vsp_version(0))
  {
    std::cerr << "Unsupported Vesperace Virtual Machine version." << std::endl;
    std::exit(EXIT_FAILURE);
  }

  VirtualMachine vm = VirtualMachine();
  vm.init_specific_system_properites();

  return vm;
}

bool VirtualMachine::is_supported_vsp_version(int version)
{
  return true;
}

void VirtualMachine::init_specific_system_properites()
{
  // this->_system_properites.push_back(new SystemProperty("123", "123"));
}


};  /*--  namespace vsp::vm  --*/

};  /*--  namespace vsp  --*/