#[derive(Debug)]
struct Student {
    roll_no: &'static str,
    name: &'static str,
    age: u32,
    marks: &'static str
}

fn main() {
    let student = Student {
        roll_no: &"333",
        name: &"Tim Bob",
        age: 13,
        marks: &"Hello"
    };
    println!("{:?}", student);
}
