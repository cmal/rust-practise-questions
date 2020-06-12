use std::ops::{ Add, Sub };
use std::ops::Not;
use std::cmp::PartialEq;
use std::convert::From;

#[derive(Debug)]
struct ComplexNumber {
    re: i32,
    im: i32
}

impl Sub<Self> for ComplexNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im
        }
    }
}

impl Add<Self> for ComplexNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl Not for ComplexNumber {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            re: self.re,
            im: - self.im
        }
    }
}

impl PartialEq<Self> for ComplexNumber {

    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

impl From<(isize, isize)> for ComplexNumber {
    fn from(t: (isize, isize)) -> Self {
        Self {
            re: t.0 as i32,
            im: t.1 as i32
        }
    }
}

trait CanMod {
    fn my_mod(&self) -> f64;
}

impl CanMod for ComplexNumber {
    fn my_mod(&self) -> f64 {
        let square = (self.re * self.re + self.im * self.im) as f64;
        square.sqrt()
    }
}

impl CanMod for (isize, isize) {
    fn my_mod(&self) -> f64 {
        let square = (self.0 * self.0 + self.1 * self.1) as f64;
        square.sqrt()
    }
}

fn mod_complex<T: CanMod>(c: T) -> f64 {
    c.my_mod()
}


fn main() {
    let c1 = ComplexNumber { re: 1, im: -2 };
    let c2 = ComplexNumber { re: -1, im: 3 };
    let c3 = ComplexNumber { re: 0, im: -4 };
    let c4 = ComplexNumber { re: 5, im: 7 };

    let c5 = ComplexNumber { re: 3, im: -3 };
    let c6 = ComplexNumber { re: 4, im: 0 };
    let c7 = ComplexNumber { re: 3, im: 3 };
    let c8 = ComplexNumber { re: 3, im: 3 };

    println!("{:?}", c1 + c2);
    println!("{:?}", c3 - c4);

    println!("{:?} == {:?} : {}", &c5, &c6, c5 == c6);
    println!("{}", c5 == !c7);
    println!("{}", c5 != !c8);

    println!("{:?}", !c5);
    println!("{:?}", !c6);

    let t = (23, 17);
    println!("{:?}", ComplexNumber::from(t));
    let c9: ComplexNumber = t.into();
    println!("{:?}", c9);

    println!("{}", mod_complex(c9));
    println!("{}", mod_complex(t));
}
