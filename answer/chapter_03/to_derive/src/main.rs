#[derive(Copy, Clone, Debug, PartialEq)]
struct Number (i32);

fn main() {
    let num = Number(8i32);
    println!("{:?}", num);

    let num_clone = num.clone();
    println!("{:?}", num_clone);

    println!("num == num_clone ? {}", num == num_clone);
}
