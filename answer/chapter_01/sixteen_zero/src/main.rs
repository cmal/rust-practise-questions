fn main() {
    let a: [i32; 16] = [0; 16];
    println!("a has {} elements", a.len());
    for e in a.iter() {
        println!("{}", e);
    }
}
