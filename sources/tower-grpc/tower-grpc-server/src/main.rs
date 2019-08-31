use log::error;
use tower_grpc_gen::aisabi::greeter::server::{Greeter, GreeterServer};
use tower_grpc_gen::aisabi::greeter::{HelloRequest, HelloReply};
use tower_grpc::{Request, Response};
use futures::future::FutureResult;
use futures::{future, Future, Stream};
use tower_hyper::Server;
use tower_hyper::server::Http;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Clone)]
struct GreeterImpl;

impl Greeter for GreeterImpl {
    type SayHelloFuture = FutureResult<Response<HelloReply>, tower_grpc::Status>;

    fn say_hello(&mut self, request: Request<HelloRequest>) -> Self::SayHelloFuture {
        println!("REQUEST = {:?}", request);

        let response = Response::new(HelloReply {
            message: "Zomg, it works!".to_string(),
        });
        future::ok(response)
    }
}

pub fn main() {
    let _ = env_logger::init();
    let new_service = GreeterServer::new(GreeterImpl);
    let mut server = Server::new(new_service);
    let http = Http::new().http2_only(true).clone();

//    let addr: SocketAddr = "[::1]:50052".parse().unwrap();
    let addr: SocketAddr = "0.0.0.0:50052".parse().unwrap();
    let bind: TcpListener = TcpListener::bind(&addr).expect("bind");

    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = server.serve_with(sock, http.clone());
            tokio::spawn(serve.map_err(|e| error!("hyper error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    tokio::run(serve);
    println!("Hello, world!");
}
