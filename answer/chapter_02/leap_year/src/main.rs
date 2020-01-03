fn is_leap_year(n: i32) -> bool {
    if n % 400 == 0 {
        true
    } else if n % 100 == 0 {
        false
    } else if n % 4 == 0 {
        true
    } else {
        false
    }
}

fn leap_year_msg(n: i32) -> String {
    format!("{} is {}leap year.", n, if is_leap_year(n) { "a " } else { "not a " })
}

fn main() {
    let mut n = 1900;
    println!("{}", leap_year_msg(n));
    n = 1984;
    println!("{}", leap_year_msg(n));
}
