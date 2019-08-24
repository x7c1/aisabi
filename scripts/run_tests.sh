#!/bin/bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

CONTAINER_NAME=aisabi_builder_cacher
IMAGE_NAME=aisabi_builder
MOUNT_DIR=/mnt
ARGS=$@

current_container() {
  docker inspect --format '{{.Created}}' ${CONTAINER_NAME}
}

run_container() {
  docker run \
      --privileged \
      --volume $(pwd):${MOUNT_DIR} \
      --name ${CONTAINER_NAME} \
      --user ${UID}:$(id -g) \
      --tty \
      --workdir ${MOUNT_DIR} \
      ${IMAGE_NAME} \
      sh ${MOUNT_DIR}/builder_internal/cargo-test.sh ${ARGS} --debug
}

restart_container() {
  docker start --attach ${CONTAINER_NAME}
}

if [ $(current_container) ]; then
  restart_container
else
  run_container
fi
