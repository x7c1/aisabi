#[macro_use]
extern crate log;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use grpc_rs_gen::{GreeterClient, HelloRequest};


fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("client started...!");

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("grpc_rs_server:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest {
        name: "world".to_string(),
        ..Default::default()
    };
//    req.set_name("world".to_owned());
    let reply = client.say_hello(&req).expect("rpc");
    info!("Greeter received: {}", reply.get_message());
}
