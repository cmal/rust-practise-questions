use std::cell::RefCell;


struct Bank {
    balance: f64,
    customer_count: RefCell<u64>,
    location: String
}

impl Bank {
    fn new(balance: f64,
           customer_count: &mut u64,
           location: String
    ) -> Self {
        Bank {
            balance: balance,
            customer_count: RefCell::new(*customer_count),
            location: location
        }
    }
}

fn main() {
    let mut a = 0u64;
    let mut b: Bank = Bank::new(0.0f64, &mut a, "Beijing".to_string());
    println!("({}, {}, {})", b.balance, b.customer_count.borrow(), b.location);
    *b.customer_count.get_mut() += 1; // if b is mut
    // *b.customer_count.borrow_mut() = 1; // if b is not mut
    println!("({}, {}, {})", b.balance, b.customer_count.borrow(), b.location);
}


// Interior mutability
// https://ricardomartins.cc/2016/06/08/interior-mutability
// https://pitdicker.github.io/Interior-mutability-patterns/
// https://www.reddit.com/r/rust/comments/a9v4g4/when_do_people_use_interior_mutability/


// Cell and RefCell
// https://blog.iany.me/2019/02/rust-cell-and-refcell/
// In Rust document, Cell is “A mutable memory location”, and RefCell is “A mutable memory location with dynamically checked borrow rules”.
// They both provide “interior mutability”, where you can modify the value stored in cell via immutable reference of the cell.
// They both have an API get_mut to return a mutable reference to the underlying data. This method requires a mutable reference to the cell, which guarantees that the callee has exclusive ownership of the cell.
// pub fn get_mut(&mut self) -> &mut T
// The difference is how they implement interior mutability. Cell copies or moves contained value, while RefCell allows both mutable and immutable reference borrowing.
