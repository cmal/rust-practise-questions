use std::cell::Cell;

// Create an immutable struct with a mutable member.
// hint: Use Cell, property known as interior mutability.

// https://www.reddit.com/r/rust/comments/755a5x/i_have_finally_understood_what_cell_and_refcell/

struct Imm<T> {
    imm_val: T,
    mut_ref: Cell<T>
}

impl<T> Imm<T> {
    fn new(a: T, b: T) -> Self {
        Imm {
            imm_val: a,
            mut_ref: Cell::new(b),
        }
    }
}

fn main() {
    let d: Imm::<usize> = Imm::new(1, 2);
    println!("({}, {})", d.imm_val, d.mut_ref.get());
    d.mut_ref.set(3);
    println!("({}, {})", d.imm_val, d.mut_ref.get());
}
