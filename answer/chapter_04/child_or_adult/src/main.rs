#[derive(Debug)]
enum Person {
    Child,
    Adult
}

fn child_or_adult(age: u8) -> Person {
    match age {
        0..=17 => Person::Child,
        _ => Person::Adult
    }
}

fn main() {
    for age in 0..20 {
        println!("At age {}, (s)he is a(n) {:?}.", age, child_or_adult(age));
    }
}
