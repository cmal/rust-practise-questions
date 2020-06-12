struct FibonacciSeries {
    curr: i32,
    next: i32
}

impl Iterator for FibonacciSeries {

    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {
        let tmp = self.curr;
        self.curr = self.next;
        self.next = tmp + self.next;
        Some(tmp)
    }
}

struct GeometricSeries {
    // first_number: i32,
    current_number: i32,
    ratio: i32
}

impl Iterator for GeometricSeries {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {

        let tmp = self.current_number;

        self.current_number = self.current_number * self.ratio;

        Some(tmp)
    }
}

fn sum<T: Iterator<Item = i32>>(iter: T, index: usize) -> i32 {
    let mut res = 0;
    for n in iter.take(index) {
        res += n;
    }
    res
}

fn main() {

    let fs1 = FibonacciSeries { curr: 1, next: 1 };
    let gs1 = GeometricSeries {
        // first_number: 1,
        current_number: 1,
        ratio: 2
    };
    println!("Fib: {}, Geo: {}", sum(fs1, 1), sum(gs1, 1));
    let fs2 = FibonacciSeries { curr: 1, next: 1 };
    let gs2 = GeometricSeries {
        // first_number: 1,
        current_number: 1,
        ratio: 2
    };
    println!("Fib: {}, Geo: {}", sum(fs2, 2), sum(gs2, 2));
    let fs3 = FibonacciSeries { curr: 1, next: 1 };
    let gs3 = GeometricSeries {
        // first_number: 1,
        current_number: 1,
        ratio: 2
    };
    println!("Fib: {}, Geo: {}", sum(fs3, 11), sum(gs3, 11));
    let fs4 = FibonacciSeries { curr: 1, next: 1 };
    let gs4 = GeometricSeries {
        // first_number: 1,
        current_number: 1,
        ratio: 2
    };
    println!("Fib: {}, Geo: {}", sum(fs4, 20), sum(gs4, 20));
}
