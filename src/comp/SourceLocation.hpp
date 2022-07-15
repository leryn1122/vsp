#pragma once
#ifndef _VSP_COMP_SOURCE_LOCATION_H_
#define _VSP_COMP_SOURCE_LOCATION_H_

#include "fwd.hpp"

namespace vsp {

namespace comp {

class FileID {
 public:
  static FileID get(int V) {
    FileID fileID;
    fileID.ID = V;
    return fileID;
  }

 private:
  int ID = 0;

 public:
  bool operator==(const FileID &rhs) const { return this->ID == rhs.ID; }
  bool operator!=(const FileID &rhs) const { return this->ID != rhs.ID; }

  bool operator<(const FileID &rhs) const { return this->ID < rhs.ID; }
  bool operator<=(const FileID &rhs) const { return this->ID <= rhs.ID; }
  bool operator>(const FileID &rhs) const { return this->ID > rhs.ID; }
  bool operator>=(const FileID &rhs) const { return this->ID >= rhs.ID; }

};  // class vsp::basic::FileID

class SourceLocation {
 private:
  size_t _rownum = 0;
  size_t _offset = 0;

 public:
  SourceLocation(size_t rownum, size_t offset) {
    _rownum = rownum;
    _offset = offset;
  }

  virtual ~SourceLocation() {}

};  // class vsp::comp::SourceLocation

};  // namespace comp

};  // namespace vsp

#endif  // _VSP_COMP_SOURCE_LOCATION_H_