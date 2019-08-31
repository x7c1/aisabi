pub mod aisabi {
    #[path = "aisabi.greeter.rs"]
    pub mod samples;

    #[path = "aisabi.health_check.rs"]
    pub mod health_check;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
