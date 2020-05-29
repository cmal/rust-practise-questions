fn main() {
    let zipper = (0..).zip("foo".chars());
    zipper.for_each(|(i, x)| println!("{}: {}", i, x));
}
