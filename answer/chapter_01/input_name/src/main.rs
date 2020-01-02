use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input).expect("error");
    println!("Hello, {}!", input);
}
