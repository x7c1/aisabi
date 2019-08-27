#[macro_use]
extern crate log;

use futures::future::Future;
use futures::sync::oneshot;
use grpc_rs_gen::{create_greeter, EchoStatus, Greeter, HelloReply, HelloRequest};
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
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
        info!("request received!!: {:?}", request);
        let message = format!("hello, {}", request.get_name());
        let response = HelloReply {
            message,
            ..Default::default()
        };
        let future = sink
            .success(response)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", request, e));;

        context.spawn(future)
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let env = Arc::new(Environment::new(1));
    let service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 50_051)
        .build()
        .unwrap();

    server.start();

    for &(ref host, port) in server.bind_addrs() {
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

fn get_status() -> EchoStatus {
    EchoStatus {
        code: 200,
        message: "hello!".to_string(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let status = get_status();
        assert_eq!(status.get_code(), 200);
        assert_eq!(status.get_message(), "hello!");
    }
}
