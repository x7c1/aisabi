use grpc_rust_gen::aaa::EchoStatus;

fn main() {
    let status = get_status();
    println!("{:#?}", status);
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
        assert_eq!(status.code, 200);
        assert_eq!(status.message, "hello!");
    }
}
