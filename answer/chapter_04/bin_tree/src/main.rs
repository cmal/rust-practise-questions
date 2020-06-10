use std::boxed::Box;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum BinTree<T: Ord + Copy> {
    Leaf {
        v: Box<T>,
        l: Box<Self>,
        r: Box<Self>
    },
    Empty
}

impl<T: Ord + Copy> BinTree<T> {
    fn new() -> Self {
        BinTree::Empty
    }
    fn add(&self, val: T) -> Self {
        match self {
            BinTree::Empty => BinTree::Leaf {
                v: Box::new(val),
                l: Box::new(BinTree::Empty),
                r: Box::new(BinTree::Empty)
            },
            BinTree::Leaf {v, l, r} => {
                match val.cmp(&v) {
                    Ordering::Less => BinTree::Leaf {
                        v: (*v).clone(),
                        l: Box::new(l.add(val)),
                        r: Box::new((**r).clone())
                    },
                    _ => BinTree::Leaf {
                        v: (*v).clone(),
                        l: Box::new((**l).clone()),
                        r: Box::new(r.add(val))
                    }
                }
            }
        }
    }
    fn min(&self) -> Option<T> {
        match self {
            BinTree::Empty => None,
            BinTree::Leaf {v, l, r: _} => {
                let min = l.min();
                match min {
                    None => Some(**v),
                    _ => min
                }
            }
        }
    }
    fn max(&self) -> Option<T> {
        match self {
            BinTree::Empty => None,
            BinTree::Leaf {v, l: _, r} => {
                let max = r.max();
                match max {
                    None => Some(**v),
                    _ => max
                }
            }
        }
    }
}

fn main() {
    let bin_tree = BinTree::<i32>::new();
    let bt = bin_tree.add(2).add(3).add(1).add(4).add(5).add(0);
    println!("max: {}", bt.max().unwrap());
    println!("min: {}", bt.min().unwrap());
    println!("{:?}", bt);
}
