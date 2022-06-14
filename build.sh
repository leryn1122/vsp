#!/usr/bin/env bash

cd -P "$(dirname "${0-$BASHSOURCE}")" || exit 1

LOG_FILE="$(date +"%Y%m%d").log"

cmake . && make