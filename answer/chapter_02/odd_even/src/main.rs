use std::io::{self, Read};

fn is_even(n:i32) -> bool {
    n % 2 == 0
}


fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    for _i in 1.. {
        let mut handle = stdin.lock();
        handle.read_to_string(&mut input).unwrap();
        let n: i32 = input.parse().unwrap();
        println!("");
        println!("{}", if is_even(n) { "even" } else { "odd" });
    }
}
