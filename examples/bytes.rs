use std::io::{BufRead, Read};

fn main() {
    let mut b = "0123456789\n".as_bytes();

    let mut buf_u8 = vec![0u8; 5];
    b.read(&mut buf_u8).unwrap();
    println!("{buf_u8:?}");

    // let mut buf_utf8 = String::with_capacity(5);
    let read = b.read_until(b'\n', &mut buf_u8).unwrap();
    println!("{read}: {buf_u8:?}");
}
