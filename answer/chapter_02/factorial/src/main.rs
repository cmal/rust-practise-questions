fn factorial(n:usize) -> usize {
    if n < 1 {
        1
    } else {
        factorial(n - 1) * n
    }
}


fn main() {
    println!("factorial(13): {}", factorial(13));
}
