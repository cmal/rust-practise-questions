use std::ops::{Add, Mul, Sub};
use std::fmt;

#[derive(Copy, Clone)]
struct Complex {
    r: i32,
    v: i32
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {:3},{:3} j )", self.r, self.v)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            r: self.r + rhs.r,
            v: self.v + rhs.v
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            r: self.r - rhs.r,
            v: self.v - rhs.v
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            r: self.r * rhs.r - self.v * rhs.v,
            v: self.r * rhs.v + self.v * rhs.r
        }
    }
}

fn main() {
    let c1 = Complex {
        r: 2,
        v: 3
    };
    let c2 = Complex {
        r: -4,
        v: 7
    };

    let c3 = Complex {
        r: 0,
        v: 5
    };
    let c4 = Complex {
        r: 11,
        v: 0
    };
    println!("{} + {} = {}", c1, c2, c1 + c2);
    println!("{} * {} = {}", c1, c3, c1 * c3);
    println!("{} - {} = {}", c4, c2, c4 - c2);
}
