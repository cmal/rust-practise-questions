use std::ops::{Add, Mul, Sub};
use std::fmt;

#[derive(Copy, Clone)]
struct Complex (i32, i32);

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {:3},{:3} j )", self.0, self.1)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex {
        Complex (self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        Complex (self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
    }
}

fn main() {
    let c1 = Complex(2, 3);
    let c2 = Complex(-4, 7);

    let c3 = Complex(0, 5);
    let c4 = Complex(11, 0);

    println!("{} + {} = {}", c1, c2, c1 + c2);
    println!("{} * {} = {}", c1, c3, c1 * c3);
    println!("{} - {} = {}", c4, c2, c4 - c2);
}
