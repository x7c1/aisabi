use grpc_rust_gen::aaa::EchoStatus;

fn main() {
    let status = get_status();
    println!("{:#?}", status);
}

fn get_status() -> EchoStatus {
    let mut status = EchoStatus::new();
    status.set_code(200);
    status.set_message("hello!".to_string());
    status
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
