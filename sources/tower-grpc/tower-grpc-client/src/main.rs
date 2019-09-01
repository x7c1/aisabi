use futures::Future;
use hyper::client::connect::{Destination, HttpConnector};
use tower_grpc::Request;
use tower_hyper::{client, util};
use tower_util::MakeService;
use tower_grpc::codegen::client::http;
use tower_grpc_gen::aisabi::greeter::client::Greeter;
use tower_grpc_gen::aisabi::greeter::HelloRequest;

pub fn main() {
    let _ = ::env_logger::init();

    let uri: http::Uri = format!("http://tower_grpc_server:50052").parse().unwrap();
    let dst = Destination::try_from_uri(uri.clone()).unwrap();
    let connector = util::Connector::new(HttpConnector::new(4));
    let settings = client::Builder::new().http2_only(true).clone();
    let mut make_client = client::Connect::with_builder(connector, settings);

    let say_hello = make_client
        .make_service(dst)
        .map_err(|e| panic!("connect error: {:?}", e))
        .and_then(move |conn| {
            let conn = tower_request_modifier::Builder::new()
                .set_origin(uri)
                .build(conn)
                .unwrap();

            // Wait until the client is ready...
            Greeter::new(conn).ready()
        })
        .and_then(|mut client| {
            client.say_hello(Request::new(HelloRequest {
                name: "What is in a name?".to_string(),
                optional_nickname: None,
                optional_address: None
            }))
        })
        .and_then(|response| {
            println!("RESPONSE = {:?}", response);
            Ok(())
        })
        .map_err(|e| {
            println!("ERR = {:?}", e);
        });

    tokio::run(say_hello);
}
