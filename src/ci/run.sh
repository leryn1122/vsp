#!/usr/bin/env bash

set -eux

if [[ -n "${CI}" ]] && [[ x"${CI}" == x"true" ]]; then
  exit 1
fi

if [[ -n "${CI_JOB_NAME}" ]]; then
  echo -e "[CI_JOB_NAME=${CI_JOB_NAME}]"
fi

ci_dir="$(cd -P "$(dirname "${0-${BASH_SOURCE[0]}}")" && pwd || exit 1)"
source "${ci_dir}/env.sh"
