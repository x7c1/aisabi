#!/bin/bash

# stop if non-zero returned.
set -e

CONTAINER_NAME=aisabi_builder_task_runner
IMAGE_NAME=aisabi_builder
MOUNT_DIR=/mnt

TARGET=$1
SCRIPTS_DIR="builder_internal"

if [[ -z ${TARGET} ]]; then
    labels=$(ls ./${SCRIPTS_DIR} | grep '.sh$' | sed 's/.sh$//')
    echo "Usage: ./scripts/run_builder_task.sh <name>"
    echo "<name>:"
    for label in ${labels}
    do
        echo "  $label"
    done
    exit 1
fi

# not allow undefined values.
set -u

# show executed commands.
set -x

# exclude $1
ARGS=${@:2}

docker run \
    --privileged \
    --volume $(pwd):${MOUNT_DIR} \
    --name ${CONTAINER_NAME} \
    --user ${UID}:$(id -g) \
    --publish 50051:50051 \
    --interactive \
    --rm \
    --tty \
    --workdir ${MOUNT_DIR} \
    ${IMAGE_NAME} \
    sh ${MOUNT_DIR}/${SCRIPTS_DIR}/${TARGET}.sh ${ARGS}
