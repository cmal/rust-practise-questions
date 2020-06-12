use num::FromPrimitive;
use std::cmp::PartialOrd;

fn greater_of_two<T: FromPrimitive + PartialOrd>(a: T, b: T) -> T {
    if a < b {
        b
    } else {
        a
    }
}

fn main() {
    println!("{}", greater_of_two(1, 2));
    println!("{}", greater_of_two(3.0, 2.0));
}
