use std::boxed::Box;
// use std::cell::{Cell, RefCell};
// use std::rc::Rc;
use std::cmp::Ordering;
// use std::borrow::Borrow;

// immutable version
#[derive(Debug, Clone)]
enum BinTree<T: Ord + Copy> {
    Leaf {
        v: T,
        l: Box<Self>,
        r: Box<Self>
    },
    Empty
}

impl<T: Ord + Copy> BinTree<T> {
    fn new() -> Self {
        BinTree::Empty
    }
    fn add(&mut self, val: T) {
        match self {
            BinTree::Empty => {
                *self = BinTree::Leaf { // NOTE: 可以直接操作指针
                    v: val,
                    l: Box::new(BinTree::Empty),
                    r: Box::new(BinTree::Empty)
                }   
            },
            // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html
            BinTree::Leaf {ref v, ref mut l, ref mut r} => { // ^^ to better know `ref`, go to the above page
                match val.cmp(v) {
                    Ordering::Less => {
                        // l.replace(Rc::new(*Rc::into_raw(l.into_inner()).add(val)));
                        l.add(val);
                    },
                    _ => {
                        // r.replace(Rc::new(*Rc::into_raw(r.into_inner()).add(val)));
                        // r.replace(Rc::new(r.into_inner().as_ref().borrow().add(val)));
                        // r.replace_with(|old| Rc::new(old.clone().as_ref().borrow().add(val)));
                        r.add(val);
                    }
                }
            }
        }
    }
    fn min(&self) -> Option<T> {
        match self {
            BinTree::Empty => None,
            BinTree::Leaf {v, ref l, r: _} => {
                let min = l.min();
                match min {
                    None => Some(*v),
                    _ => min
                }
            }
        }
    }
    fn max(&self) -> Option<T> {
        match self {
            BinTree::Empty => None,
            BinTree::Leaf {v, l: _, ref r} => {
                let max = r.max();
                match max {
                    None => Some(*v),
                    _ => max
                }
            }
        }
    }
}

fn main() {
    let mut bt = BinTree::<i32>::new();
    bt.add(2);
    bt.add(3);
    bt.add(1);
    bt.add(4);
    bt.add(5);
    bt.add(0);
    println!("max: {}", bt.max().unwrap());
    println!("min: {}", bt.min().unwrap());
    println!("{:?}", bt);
}
