fn print_mountain(n:usize) {
    for i in 1..=n {
        println!("{}{}", " ".repeat(n - i), "*".repeat(2 * i - 1));
    }
}


fn main() {
    print_mountain(7);
}
