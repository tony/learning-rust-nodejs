// Pure Rust logic that can be tested without Node.js/V8 dependencies

pub fn get_hello_message() -> &'static str {
    "hello node"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_message() {
        assert_eq!(get_hello_message(), "hello node");
    }
}
