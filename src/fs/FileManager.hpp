#pragma once
#ifndef _VSP_FS_FILE_MANAGER_H_
#define _VSP_FS_FILE_MANAGER_H_

#include "fwd.hpp"

namespace vsp {

namespace fs {

class FileManager;

/// Simplified directory entry.
class DirectoryEntry {
  friend class FileManager;

  std::string name;

 public:
  std::string get_name() const { return name; }
}

/// Simplified file entry with necessary info.
class FileEntry {
  friend class FileManager;

 private:
  std::string name;
  std::string real_pathname;
  long last_modified;
  unsigned uid;
  const DirectoryEntry *dir;
  // mutable std::unique_ptr<fs::File> file;

 public:
  FileEntry() {}
  FileEntry(const FileEntry &) = delete;

  std::string get_name() const { return this->name; }
  std::string get_real_pathname() const { return this->real_pathname; }
  long get_last_modified() const { return this->last_modified; }
  const DirectoryEntry *get_dir() const { return dir; }
}

class FileManager {
 private:
  FileSystemOptions file_system_options;

  std::map<std::string, DirectoryEntry> cached_dirs;
  std::map<std::string, FileEntry> cached_files;

 public:
  FileManager(const FileSystemOptions file_system_options);
  virtual ~FileManager() {}

  vsp::Expected<DirectoryEntry> get_dir_ref();

  vsp::Expected<FileEntry> get_file_ref();

  bool make_absolute_path() const;

  void print_stats();

};  // class vsp::fs::FileManager

};  // namespace fs

};  // namespace vsp

#endif  // _VSP_FS_FILE_MANAGER_H_