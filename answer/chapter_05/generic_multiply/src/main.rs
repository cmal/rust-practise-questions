use fsize::fsize;
use num::FromPrimitive;
use std::ops::Mul;

fn multiply<T: FromPrimitive + Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

fn main() {
    let u: usize = 3;
    let i: isize = 5;
    let f: fsize = 7.0;
    println!("{}", multiply(u, u));
    println!("{}", multiply(i, i));
    println!("{}", multiply(f, f));
}
