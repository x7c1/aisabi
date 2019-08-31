extern crate protoc_grpcio;

fn main() {
    let gen_dir = "/mnt/sources/grpc-rs/grpc-rs-gen";
    let proto_root = "/mnt/sources/proto";
    println!("cargo:rerun-if-changed={}", proto_root);

    protoc_grpcio::compile_grpc_protos(
        &[
            "/mnt/sources/proto/greeter.proto",
        ],
        &[proto_root, "/usr/local/include"],
        &(gen_dir.to_owned() + "/src"),
        None,
    )
    .expect("Failed to compile gRPC definitions!");
}
