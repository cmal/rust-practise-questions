#[derive(Debug)]
struct GeometricSeries {
    first_number: i32,
    current_number: i32,
    ratio: i32
}

impl Iterator for GeometricSeries {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        
        self.current_number = self.current_number * self.ratio;
        
        Some(self.current_number)
    }
}

fn main() {
    let gs = GeometricSeries {
        first_number: 1,
        current_number: 1,
        ratio: 2
    };

    for n in gs.take(11) {
        println!("{:?}", n);
    }
}
