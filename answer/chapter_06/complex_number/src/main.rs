use std::ops::{ Add, Sub };

#[derive(Debug)]
struct ComplexNumber {
    re: i32,
    im: i32
}

impl Sub<ComplexNumber> for ComplexNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im
        }
    }
}

impl Add<ComplexNumber> for ComplexNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}


fn main() {
    let c1 = ComplexNumber { re: 1, im: -2 };
    let c2 = ComplexNumber { re: -1, im: 3 };
    let c3 = ComplexNumber { re: 0, im: -4 };
    let c4 = ComplexNumber { re: 5, im: 7 };

    println!("{:?}", c1 + c2);
    println!("{:?}", c3 - c4)
}
