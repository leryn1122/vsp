#!/usr/bin/env bash

cd -P "$(dirname "${0-$BASHSOURCE}")" || exit 1

if command -v clang >/dev/null ; then
  export  CC="$(which clang)" \
         CXX="$(which clang)"
else
  export  CC="$(which gcc)" \
         CXX="$(which g++)"
fi

cmake . -B build && cd build && make