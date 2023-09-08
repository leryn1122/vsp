#!/usr/bin/env bash

set -eu

declare extra_arg;
declare build_py;

work_dir="$(cd -P "$(dirname "$(readlink -f "${0-${BASH_SOURCE[0]}}")")" && pwd || exit 1)"
cd -P "${work_dir}" && pwd || exit 1
build_py="./main.py"

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
