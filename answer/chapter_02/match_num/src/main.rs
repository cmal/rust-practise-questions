use std::io::{self, Read};

fn text_n(n:i32) -> String {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "NaN",
    }.to_string()
}


fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    println!("");
    println!("{}", text_n(n));
}
