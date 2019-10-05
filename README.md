# aisabi [![Build Status](https://travis-ci.com/x7c1/aisabi.svg?branch=master)](https://travis-ci.com/x7c1/aisabi)

Compare gRPC libraries written in Rust.

* [sources/grpc-rust](sources/grpc-rust)
  * using [stepancheg/grpc-rust: Rust implementation of gRPC](https://github.com/stepancheg/grpc-rust)
* [sources/grpc-rs](sources/grpc-rs)
  * using [pingcap/grpc-rs: The gRPC library for Rust built on C Core library and futures](https://github.com/pingcap/grpc-rs)
* [sources/tower-grpc](sources/tower-grpc)
  * using [tower-rs/tower-grpc: A gRPC client & server implementation.](https://github.com/tower-rs/tower-grpc)

## Requirement

Install `docker` and `docker-compose` first.

## Build

Create docker container to build crates:

```
$ ./scripts/setup-protoc-docker.sh
```

Build crates and test them (on the container):

```
$ ./scripts/run_tests.sh
```

## Run

grpc-rs

```
$ docker-compose up grpc_rs_client
...
grpc_rs_client_1     | [2019-10-05T05:04:48Z INFO  grpc_rs_client] client started...!
grpc_rs_client_1     | [2019-10-05T05:04:48Z INFO  grpc_rs_client] Greeter received: hello, world!
aisabi_grpc_rs_client_1 exited with code 0
```

tower-grpc

```
$ docker-compose up tower_grpc_client
...
tower_grpc_client_1  | RESPONSE = Response { metadata: MetadataMap { headers: {"content-type": "application/grpc+proto", "date": "Sat, 05 Oct 2019 05:05:24 GMT"} }, message: HelloReply { message: "Zomg, it works!" } }
aisabi_tower_grpc_client_1 exited with code 0
```
