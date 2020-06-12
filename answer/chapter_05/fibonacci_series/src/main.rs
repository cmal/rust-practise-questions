struct FibonacciSeries {
    curr: u32,
    next: u32
}

impl Iterator for FibonacciSeries {

    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        let tmp = self.curr;
        self.curr = self.next;
        self.next = tmp + self.next;
        Some(tmp)
    }
}


fn main() {
    let fs = FibonacciSeries { curr: 1, next: 1 };
    for n in fs.take(11) {
        println!("{}", n);
    }
}
