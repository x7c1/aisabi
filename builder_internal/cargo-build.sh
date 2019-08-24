#!/usr/bin/env bash

get_opt_level() {
  while getopts ":-:" OPT; do
    case "${OPT}" in
      -)  case "${OPTARG}" in
            opt-level=*)
              echo ${OPTARG#*=}
              exit
              ;;
          esac
          ;;
    esac
  done
  echo 2
}

get_build_mode() {
  while getopts ":-:" OPT; do
    case "${OPT}" in
      -)  case "${OPTARG}" in
            debug)
              echo ""
              exit
              ;;
          esac
          ;;
    esac
  done
  echo "--release"
}

# show executed commands.
set -x

cd /mnt

RUSTFLAGS="-C opt-level=$(get_opt_level $@)" \
cargo build \
    --verbose \
    $(get_build_mode $@)
