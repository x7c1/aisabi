use grpc_rust_gen::greeter::HelloRequest;

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
