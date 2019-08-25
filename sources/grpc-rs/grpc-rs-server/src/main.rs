use grpc_rs_gen::EchoStatus;

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
        assert_eq!(status.get_code(), 200);
        assert_eq!(status.get_message(), "hello!");
    }
}
