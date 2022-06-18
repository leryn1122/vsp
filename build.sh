#!/usr/bin/env bash

cd -P "$(dirname "${0-$BASHSOURCE}")" || exit 1

if command -v gcc >/dev/null ; then
  export CC="$(which gcc)" \
         CXX="$(which g++)" 
else
  export CC="$(which clang)" \
         CXX="$(which clang)"
fi

LOG_FILE="$(date +"%Y%m%d").log"

cmake . && make

$(cd release && tree -a . > "../${LOG_FILE}") 