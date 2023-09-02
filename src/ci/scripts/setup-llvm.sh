#!/usr/bin/env bash

set -eux

LLVM_VERSION=14
DISTRO=$(lsb_release -is)
VERSION=$(lsb_version -sr)

function install_llvm() {
  case $DISTRO in
    Ubuntu|Debian)
      apt_install
      ;;
    *)
      exit 1
  esac
}

function apt_install() {
  local repo_name
  repo_name="deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-${LLVM_VERSION} main"
  wget -qO - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
  add-apt-repository "${repo_name}" -y
  apt-get update && apt-get install -y \
    llvm-$LLVM_VERSION         \
    llvm-$LLVM_VERSION-*       \
    liblld-$LLVM_VERSION*      \
    libpolly-$LLVM_VERSION-dev
}

install_llvm
