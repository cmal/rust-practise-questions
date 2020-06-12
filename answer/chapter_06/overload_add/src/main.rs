use fsize::fsize;
use std::fmt::{ Result, Formatter, Display };
use std::ops::Add;

struct Num {
    int: isize,
    flt: fsize
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.int as f64 + self.flt as f64)
    }
}

impl Add<Self> for Num {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            int: self.int + rhs.int,
            flt: self.flt + rhs.flt
        }
    }
}

fn main() {
    let n1 = Num { int: 3, flt: 0.0 };
    let n2 = Num { int: 0, flt: 2.5 };
    println!("{}", n1 + n2);
}
