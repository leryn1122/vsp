#!/usr/bin/env bash

set -eu

channel="$(cd -P "$(dirname "${0-${BASH_SOURCE[0]}}")" && pwd || exit 1)/../channel"
channel="$(cat "$channel")"

case "${channel}" in
  stable)
    channel_branch="master"
    ;;
  nightly)
    channel_branch="nightly"
    ;;
  unstable)
    channel_branch="unstable"
    ;;
  *)
    echo -e "Error: Fail to fetch channel [${channel}]" >&2
    exit 1
esac

echo "${channel_branch}"
