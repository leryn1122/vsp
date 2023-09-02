#!/bin/bash
#shellchek=bash

set -eu

:<<USAGE-HEREDOC
USAGE:
  vsp-init [OPTION [...] ]

OPTION:

  -y
      Disable confirmation prompt.

  -h, --help
      Print help info.

USAGE-HEREDOC

VSP_UPDATE_ROOT="${VSP_UPDATE_ROOT:-https://dl.vespera.io}"

# Print the heredoc above quoted by `USAGE-HEREDOC`
print_usage() {
  sed -n -e '/USAGE-HEREDOC$/,/^USAGE-HEREDOC$/p' "$0" | \
    sed -e '/USAGE-HEREDOC$/d'
}

# Print stack trace, implemented by built-in `caller`.
stack_trace() {
  printf 'error: %s\n' "$1" >&2
  local i=0
  while true; do
    s=$(caller $i)
    if [ -z "$s" ]; then
      break
    fi
    # shellcheck disable=SC2068
    printf '  at %s:%s (File: %s)\n' ${s[@]};
    ((i++))
  done
}

# Print stack trace, and throw error.
throw() {
  stack_trace "$1" >&2
  exit 1
}

# Ensure the command executed successfully, or throw error.
ensure() {
  if ! "$@"; then
    throw "command failed: $*";
  fi
}

# Determine whether the command exists, or throw error.
require_cmd() {
  if ! command -v "$1" > /dev/null 2>&1; then
    throw "Command not found: \`$1\`"
  fi
}

# Determine whether the command exists, and return 1 if missing.
check_cmd() {
  if command -v "$1" > /dev/null 2>&1; then
    return 0
  else
    return 1
  fi
}

# Obtain host triplet, such as `x86_64-apple-darwin`.
obtain_triplet() {
  local _osType _osType _clibType

  _osType="$(uname -s)"
  _cpuType="$(uname -m)"
  _clibType="gnu"

  if [[ "${_osType}" == "Darwin" ]]; then
    :
  fi

  if [[ "${_osType}" == "Linux" ]]; then
    if ldd --version 2>&1 | grep -q 'musl'; then
      _clibType="musl"
    fi
  fi

  case "${_osType}" in
    Darwin)
      _osType="apple-darwin"
      ;;
    MINGW* | MSYS* | CYGWIN* )
      _osType="pc-windows-gnu"
      ;;
    *)
      throw "unsupported OS type: ${_osType}"
  esac

  case "${_cpuType}" in
    aarch64 | arm64)
      _cpuType="aarch64"
      ;;
    x86_64 | x86-64 | x64 | amd64)
      _cpuType="x86_64"
      ;;
    *)
      throw "unsupported CPU type: ${_cpuType}"
  esac

  echo "${_cpuType}-${_osType}"
}

# Detect download client, which shall be one of `curl`` or `wget` client.
detect_download_client() {
  local _dld
  if check_cmd curl ; then
    _dld="curl"
  elif check_cmd wget; then
    _dld="wget"
  else
    throw "missing downloader client: one of \`curl\` and \`wget\` required"
  fi
  echo "${_dld}"
}

# @Params URL where the executable located.
# @Params Executable filename.
# @Params Directory where to download.
download_setup() {
  local _dld _err _status
  _dld="$(detect_download_client)"

  if [[ "${_dld}" == "curl" ]] ; then
    _err=$(curl \
      --proto '=https' \
      --tlsv1.2 \
      --silent \
      --show-error \
      --fail \
      --location "$1" \
      --output "$2" \
      2>&1)
    _status=$?
  elif [[ "${_dld}" == "wget" ]] ; then
    local _Busybox
    _Busybox="$(wget -V 2>&1 | grep -c -iE 'busybox')"
    if [[ "${_Busybox}" -ge 0 ]] ; then
      # using busybox version of wget.
      _err=$(wget "$1" -O "$2" 2>&1)
      _status=$?
    else
      _err=$(wget \
        --https-only \
        --secure-protocol=TLSv1_2 \
        "$1" -O "$2" 2>&1)
      _status=$?
    fi
  else
    throw "unsupported downloader client: ${_dld}"
  fi

  if [[ -n "${_err}" ]] ; then
    throw "${_err}"
  fi
  return "${_status}"
}

main() {
  require_cmd uname
  require_cmd mktemp
  require_cmd mkdir

  local _triplet
  _triplet="$(obtain_triplet)"

  local _ext=""
  if [[ "${_triplet}" =~ "windows" ]] ; then
    _ext=".exe"
  fi

  local _url
  _url="${VSP_UPDATE_ROOT}/dist/${_triplet}/vsp-init${_ext}"

  local _dir
  local _file

  if ! _dir="$(ensure mktemp -d)"; then
    exit 1
  fi
  _file="${_dir}/vsp-init${_ext}"

  local _needTTY
  for arg in "$@"; do
    case "${arg}" in
      --help)
        print_usage
        exit 0
        ;;
      *)
        OPTIND=1
        if [[ "${arg%%--*}" = "" ]]; then
          continue
        fi
        while getopts ':hy' opt "${arg}"; do
          case "${opt}" in
            h)
              print_usage
              exit 0
            ;;
            y)
              # Skip the prompt which ask for yes
              _needTTY=true
              ;;
            *)
              ;;
          esac
        done
        ;;
    esac
  done

  ensure mkdir -p "${_dir}"
  ensure download_setup "${_url}" "${_file}" "${_triplet}"
  ensure chmod u+x "${_file}"

  if [[ ! -x "${_file}" ]]; then
    throw "failed to execute binary: ${_file}"
  fi

  if [[ "${_needTTY}" == "yes" ]] && [[ ! -t 0 ]]; then
    if [[ ! -t 1 ]]; then
      throw "failed to "
    fi

    "${_file}" "$@" < /dev/tty
  else
    "${_file}" "$@"
  fi

  local _ret=$?

  # Clean temporary files.
  [[ -f "${_file}" ]] && rm "${_file}"
  [[ -d "${_dir}" ]] && rmdir "${_dir}"

  return "${_ret}"
}

main "$@" || exit 1
