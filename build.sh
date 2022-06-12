#!/usr/bin/env bash

cd -P "$(dirname "${0-$BASHSOURCE}")" || exit 1

cmake . && make