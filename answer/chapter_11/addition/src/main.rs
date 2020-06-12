macro_rules! addition {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        $x + addition!($($y),+)
    )
}


fn main() {
    println!("{}", addition!(5, 6, 57, 56, 1));
    assert_eq!(13, addition!(4, 9));
}
// * Implement a macro named `addition` to add any amount of numbers. `Eg`: `addition!(5, 6, 57 ,56, 1)`
// should return `125` and `addition!(4, 9)` should return `11`.
