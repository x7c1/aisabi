use grpc_rust_gen::greeter::HelloRequest;

/*
use grpc_rust_gen::greeter::HelloReply;
use grpc_rust_gen::greeter_grpc::Greeter;

// cannot build

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(&self, _: ServerHandlerContext, req: ServerRequestSingle<HelloRequest>, resp: ServerResponseUnarySink<HelloReply>) -> grpc::Result<()> {
        let mut r = HelloReply::new();
        let name = if req.message.get_name().is_empty() {
            "world"
        } else {
            req.message.get_name()
        };
        println!("greeting request from {}", name);
        r.set_message(format!("Hello {}", name));
        resp.finish(r)
    }
}
*/

fn main() {
    let status = get_request();
    println!("{:#?}", status);
}

fn get_request() -> HelloRequest {
    HelloRequest {
        name: "world".to_string(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let request = get_request();
        assert_eq!(request.name, "world");
    }
}
