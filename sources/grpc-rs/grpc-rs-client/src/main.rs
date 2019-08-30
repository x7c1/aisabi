#[macro_use]
extern crate log;

use std::sync::Arc;

use grpc_rs_gen::greeter::HelloRequest;
use grpc_rs_gen::greeter::HelloRequest_oneof_optional_address::address;
use grpc_rs_gen::greeter_grpc::GreeterClient;
use grpcio::{ChannelBuilder, EnvBuilder, Environment};

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("client started...!");

    //    let env = Arc::new(EnvBuilder::new().build());
    let env = Arc::new(Environment::new(1));

    let channel = ChannelBuilder::new(env).connect("grpc_rs_server:50051");
    let client = GreeterClient::new(channel);

    let request = HelloRequest {
        name: "world".to_string(),
        optional_address: Some(address("hoge addr".to_string())),
        ..Default::default()
    };
    let reply = client.say_hello(&request).expect("rpc");
    info!("Greeter received: {}", reply.get_message());
}
