use std::ops::Add;

#[derive(Debug)]
struct Number {
    val: f64
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        Number {
            val: self.val + rhs.val
        }
    }
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { val: item as f64 }
    }
}

impl From<f64> for Number {
    fn from(item: f64) -> Self {
        Number { val: item }
    }
}

fn main() {

    let a = Number::from(6i32);
    let b = Number::from(7.0f64);
    let c = Number::from(7.0f64);
    let d = Number::from(6i32);

    println!("a(6i32) + b(7.0f64) is {:?}", a + b);
    println!("c(7.0f64) + d(6i32) is {:?}", c + d);
    
}
