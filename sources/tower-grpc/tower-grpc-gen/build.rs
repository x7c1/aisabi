use prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.out_dir("./src/aisabi");

    tower_grpc_build::Config::from_prost(config)
        .enable_server(true)
        .enable_client(true)
        .build(
            &[
                "../../proto/greeter.proto",
                "../../proto/health_check.proto",
            ],
            &["../../proto"],
        )
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    println!("cargo:rerun-if-changed=../../proto/greeter.proto");
    println!("cargo:rerun-if-changed=../../proto/health_check.proto");
}
