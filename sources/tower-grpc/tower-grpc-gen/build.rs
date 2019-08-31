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

//    // Build helloworld
//    tower_grpc_build::Config::new()
//        .enable_server(true)
//        .enable_client(true)
//        .build(
//            &["proto/helloworld/helloworld.proto"],
//            &["proto/helloworld"],
//        )
//        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
//    println!("cargo:rerun-if-changed=proto/helloworld/helloworld.proto");

    // Build metadata
//    tower_grpc_build::Config::new()
//        .enable_server(true)
//        .enable_client(true)
//        .build(&["proto/metadata/metadata.proto"], &["proto/metadata"])
//        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
//    println!("cargo:rerun-if-changed=proto/metadata/metadata.proto");
//
//    // Build routeguide
//    tower_grpc_build::Config::new()
//        .enable_server(true)
//        .enable_client(true)
//        .build(
//            &["proto/routeguide/route_guide.proto"],
//            &["proto/routeguide"],
//        )
//        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
//    println!("cargo:rerun-if-changed=proto/routeguide/route_guide.proto");
}
