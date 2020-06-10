#[derive(Debug)]
enum Grade {
    Passed,
    NotPassed
}

fn is_passed(score: f64) -> Grade {
    match score {
        x if x < 60.0 => Grade::NotPassed,
        _ => Grade::Passed
    }
}

fn main() {
    for score in 58..62 {
         println!("{} is {:?}", score, is_passed(score as f64));
    }
    const SC: f64 = 59.5;
    println!("{} is {:?}", SC, is_passed(SC as f64));
}
