use std::cmp::Ordering;

fn min_of_two<T: Ord>(a: T, b: T) -> T {
    match a.cmp(&b) {
        Ordering::Less => a,
        _ => b
    }
}

fn main() {
    println!("{}", min_of_two(1, 2));
    println!("{}", min_of_two(false, true));
    println!("{}", min_of_two("xx", "aa"));
}
