extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: &("./src"),
        includes: &["../../proto", "/usr/local/include"],
        input: &[
            "../../proto/greeter.proto",
            "../../proto/health_check.proto",
        ],
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc-rust-grpc");

    println!("cargo:rerun-if-changed={}", "../../proto");
}
