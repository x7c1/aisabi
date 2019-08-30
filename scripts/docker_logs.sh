#!/usr/bin/env bash

# [seconds]
export COMPOSE_HTTP_TIMEOUT=$((60 * 60))

docker-compose logs --timestamp --follow
