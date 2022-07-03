#pragma once
#ifndef _VSP_LANG_SOURCE_LOCATION_H_
#define _VSP_LANG_SOURCE_LOCATION_H_


#include "fwd.hpp"

namespace vsp
{

namespace lang
{

class FileID
{

private:
  int ID = 0;

public:
  bool operator==(const FileID &rhs) const { return this->ID == rhs.ID; }
  bool operator!=(const FileID &rhs) const { return this->ID != rhs.ID; }

  bool operator< (const FileID &rhs) const { return this->ID < rhs.ID; }
  bool operator<=(const FileID &rhs) const { return this->ID <= rhs.ID; }
  bool operator> (const FileID &rhs) const { return this->ID > rhs.ID; }
  bool operator>=(const FileID &rhs) const { return this->ID >= rhs.ID; }


};  /*--  class vsp::lang::FileID  --*/

};  /*--  namespace vsp::lang  --*/

};  /*--  namespace vsp  --*/

#endif  /*--  _VSP_LANG_SOURCE_LOCATION_H_  --*/