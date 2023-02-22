use std::io;
use ascii_crypt::decode;
use ascii_crypt::encode;

fn main() {
    let mut input = String::new();
    println!("enter string to encode");
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", encode(input.trim()).unwrap());
    input = String::new();
    println!("enter string to decode");
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", decode(input.trim()).unwrap());
}
