

fn main() {
    let mut v: Vec<i32> = (0..=100).collect();
    v.retain(|n| n % 3 != 0);
    println!("{:?}", v);
}
