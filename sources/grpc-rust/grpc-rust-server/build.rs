extern crate protoc_rust_grpc;

fn main() {
    let gen_dir = "/mnt/sources/grpc-rust/grpc-rust-gen";

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: &(gen_dir.to_owned() + "/src"),
        includes: &["/mnt/sources/proto", "/usr/local/include"],
        input: &["/mnt/sources/proto/aaa.proto"],
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc-rust-grpc");
}
