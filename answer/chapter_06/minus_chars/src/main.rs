use std::vec::Vec;
use std::ops::Sub;

#[derive(Debug)]
struct Set {
    chars: Vec<char>
}

impl Sub<Set> for Set {
    type Output = Set;

    fn sub(self, rhs: Set) -> Set {
        let mut s = Set { chars: Vec::new() };
        for &c in self.chars.iter() {
            match rhs.chars.iter().find(|&&c1| c == c1) {
                None => s.chars.push(c),
                _ => {}
            };
        }
        s
    }
}

fn main() {
    let s1 = Set {
        chars: vec!('a', 'e', 'i', 'o', 'u')
    };

    let s2 = Set {
        chars: vec!('a', 'b', 'c', 'd', 'e')
    };

    println!("{:?}", s1);
    println!("{:?}", s2);

    println!("{:?}", s1 - s2);
    // println!("{:?}", s2 - s1);
}
