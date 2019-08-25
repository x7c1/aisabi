#!/usr/bin/env bash

set -x

IMAGE_NAME=aisabi_builder

docker build ./builder_docker \
    --tag ${IMAGE_NAME}
