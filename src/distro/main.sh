#!/usr/bin/env bash

set -eux

declare extra_arg;
declare build_py;

build_py="$(cd -P "$(dirname "${0-${BASH_SOURCE[0]}}")" && pwd || exit 1)/distro.py"

SEARCH_PYTHON=("python3" "py" "python" "python2")
for search_python in "${SEARCH_PYTHON[@]}" ; do
  if python=$(command -v "$search_python") && [ -x "$python" ] ; then
    # Use `py -3 ...` if `py` was found.
    if [ "$search_python" = "py" ] ; then
      extra_arg="-3"
    else
      extra_arg=""
    fi
    # shellcheck disable=SC2086
    exec "$search_python" $extra_arg "$build_py" "$@"
  fi
done

echo "Failed: \`python' not installed when calling ${0-${BASH_SOURCE[0]}}" >&2
exit 1
