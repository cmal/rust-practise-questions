fn swap(i: &mut i32, j: &mut i32) {
    // TODO
}

fn main() {
    let mut i = 3;
    let mut j = 4;
    println!("i and j before swap: {}, {}", i, j);
    swap(&mut i, &mut j);
    println!("i and j after swap: {}, {}", i, j);
}
