#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

. $(dirname $0)/setup-env.sh

cd /mnt

# build single crate: grpc-rs-gen
# cargo build --target-dir sources/grpc-rs/grpc-rs-gen

RUSTFLAGS="-C opt-level=$(get_opt_level $@)" \
cargo build \
  $(get_build_mode $@)

cargo test \
  $(get_build_mode $@) \
  -- --nocapture
