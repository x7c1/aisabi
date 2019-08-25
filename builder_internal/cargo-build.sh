#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

. $(dirname $0)/setup-env.sh

cd /mnt

RUSTFLAGS="-C opt-level=$(get_opt_level $@)" \
cargo build \
    --verbose \
    $(get_build_mode $@)
