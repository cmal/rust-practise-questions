// std::cmp::PartialOrd;
use std::cmp::Ordering;

#[derive(Eq)]
struct Class {
    size: u32,
    section: u32,
    grade: u32
}

// impl Eq for Class {}

impl PartialOrd for Class {
    fn partial_cmp(&self, rhs: &Class) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Class {
    fn cmp(&self, rhs: &Class) -> Ordering {
        self.size.cmp(&rhs.size)
    }
}

impl PartialEq for Class {
    fn eq(&self, rhs: &Class) -> bool {
        self.size == rhs.size
    }
}



fn main() {
    let c1 = Class {
        size: 1,
        section: 1,
        grade: 2
    };

    let c2 = Class {
        size: 1,
        section: 2,
        grade: 2
    };

    let c3 = Class {
        size: 2,
        section: 2,
        grade: 2
    };

    println!("{}", c1 == c2);
    println!("{}", c1 < c2);
    println!("{}", c2 < c3);
    println!("{}", c1 <= c2);
    println!("{}", c2 <= c3);
}
