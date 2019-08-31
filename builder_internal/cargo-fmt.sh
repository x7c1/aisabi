#!/usr/bin/env bash

# show executed commands.
set -x

cd /mnt

while getopts ":-:" OPT
do
  case ${OPT} in
    -)  case "${OPTARG}" in
          all-check)
            cargo fmt --verbose --all -- --check
            ;;
          emit-files)
            cargo fmt --verbose -- --emit files
            ;;
          help)
            cargo fmt --help
            rustfmt --help
            rustfmt --help=config
            rustfmt --help=file-lines
            ;;
        esac
        ;;
  esac
done
