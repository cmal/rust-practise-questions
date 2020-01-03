fn add(i:i32, j:i32) -> i32 {
    i + j
}

fn main() {
    let i = 3;
    let j = 4;
    println!("sum of {} and {} is: {}", i, j, add(i,j));
}
