#[macro_use]
extern crate log;

use futures::future::Future;
use futures::sync::oneshot;
use grpc_rs_gen::aaa::EchoStatus;
use grpc_rs_gen::greeter::HelloRequest_oneof_optional_address::address;
use grpc_rs_gen::greeter::{HelloReply, HelloRequest};
use grpc_rs_gen::greeter_grpc::{create_greeter, Greeter, GreeterClient};
use grpcio::{ChannelBuilder, Environment, RpcContext, Server, ServerBuilder, UnarySink};
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

// https://github.com/pingcap/grpc-rs/blob/master/tests-and-examples/examples/hello_world/server.rs

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(
        &mut self,
        context: RpcContext,
        request: HelloRequest,
        sink: UnarySink<HelloReply>,
    ) {
        info!("request received...: {:#?}", request);
        let message = format!("hello, {}!", request.get_name());

        if request.has_address() {
            info!("address exists: {}", request.get_address());
        }
        let response = HelloReply {
            message,
            ..Default::default()
        };
        let future = sink
            .success(response)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", request, e));
        ;

        context.spawn(future)
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut server = setup_server();

    for (host, port) in server.bind_addrs() {
        info!("listening on... {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });

    let _ = rx.wait();
    let _ = server.shutdown().wait();
}

fn setup_server() -> Server {
    let env = Arc::new(Environment::new(1));
    let service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 50051)
        .build()
        .unwrap();

    server.start();
    server
}

#[test]
fn test_greeter_service() {
    let server = setup_server();

    let (host, port) = &server.bind_addrs()[0];
    let env = Arc::new(Environment::new(1));
    let channel = ChannelBuilder::new(env).connect(&format!("0.0.0.0:{}", port));
    let client = GreeterClient::new(channel);

    let mut request = HelloRequest {
        name: "world".to_string(),
        optional_address: Some(address("hoge addr".to_string())),
        ..Default::default()
    };
    let request = {
        let mut x = HelloRequest::new();
        x.set_name("world".to_string());
        x.set_address("sample address".to_string());
        x
    };
    println!("---host, {} {:?}", host, request);
    let reply = client.say_hello(&request).unwrap();
    assert_eq!(reply.message, "hello, world!")
}
