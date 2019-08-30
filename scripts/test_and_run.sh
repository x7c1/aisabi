#!/bin/bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

./scripts/run_tests.sh

docker-compose up grpc_rs_client
