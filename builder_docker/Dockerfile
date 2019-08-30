FROM rust:1.37

# https://github.com/protocolbuffers/protobuf/releases/
ARG pb_version="3.9.1"

RUN set -x && \
  curl -OL https://github.com/google/protobuf/releases/download/v${pb_version}/protoc-${pb_version}-linux-x86_64.zip

RUN set -x && \
  unzip protoc-${pb_version}-linux-x86_64.zip -d protoc3 && \
  mv protoc3/bin/* /usr/local/bin/ && \
  mv protoc3/include/* /usr/local/include/

RUN set -x &&\
  chmod ugo+x /usr/local/bin/protoc && \
  chmod ugo+r /usr/local/include/google && \
  chmod ugo+r /usr/local/include/google/**/*

RUN set -x && \
  apt-get update && \
  apt-get install -y \
    cmake \
    lsof \
    golang-go

RUN set -x && \
  rustup component add rustfmt --toolchain 1.37.0-x86_64-unknown-linux-gnu
