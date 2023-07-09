#!/usr/bin/env bash

set -eux

LLVM_VERSION=14
DISTRO=$(lsb_release -is)
VERSION=$(lsb_version -sr)

case $DISTRO in
  Ubuntu|Debian)
    REPO_NAME="deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-${LLVM_VERSION} main"
    ;;
  *)
    exit 1
esac

wget -qO - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
add-apt-repository "${REPO_NAME}" --yes
apt-get update && apt-get install -y \
  llvm-$LLVM_VERSION \
  llvm-$LLVM_VERSION-* \
  liblld-$LLVM_VERSION* \
  libpolly-$LLVM_VERSION-dev
