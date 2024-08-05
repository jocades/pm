use std::io::{self, BufReader, Read};

// request:
// > start { ...options }\n
// < 1 ...
// < 1 ...
fn main() {
    let request = "> start {\"process\": \"server.js\"}".as_bytes();
}
