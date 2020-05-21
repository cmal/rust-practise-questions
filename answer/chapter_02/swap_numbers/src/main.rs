fn swap(i: &mut i32, j: &mut i32) {
    let tmp = i.clone();
    *i = j.clone();
    *j = tmp;
}

fn main() {
    let i: &mut i32 = &mut 3;
    let j: &mut i32 = &mut 4;
    println!("i and j before swap: {}, {}", i, j);
    swap(i, j);
    println!("i and j after swap: {}, {}", i, j);
}
