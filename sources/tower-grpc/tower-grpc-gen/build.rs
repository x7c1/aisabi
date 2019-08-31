use prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.out_dir("/mnt/sources/tower-grpc/tower-grpc-gen/src/aisabi");

    tower_grpc_build::Config::from_prost(config)
        .enable_server(true)
        .enable_client(true)
        .build(
            &["/mnt/sources/proto/greeter.proto"],
            &["/mnt/sources/proto"],
        )
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    println!("cargo:rerun-if-changed=/mnt/sources/proto/greeter.proto");
}
