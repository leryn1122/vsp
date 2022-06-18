#!/usr/bin/env bash

cd -P "$(dirname "${0-$BASHSOURCE}")" || exit 1

export CC="$(which clang)"
export CXX="$(which clang)" 

LOG_FILE="$(date +"%Y%m%d").log"

cmake . && make
