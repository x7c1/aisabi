use grpc_rust_gen::aaa::EchoStatus;

fn main() {
    let status = {
        let mut status = EchoStatus::new();
        status.set_code(200);
        status.set_message("hello!".to_string());
        status
    };
    println!("{:#?}", status);
}
