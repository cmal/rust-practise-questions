use std::io;
use std::io::prelude::*; 
use regex::{Regex, Captures};

#[derive(Debug)]
#[derive(PartialEq)]
struct Date {
    year: String,
    month: String,
    day: String
}

fn to_date(date: &str) -> Date {
    let re = Regex::new(r"(?P<y>\d{4})\D*(?P<m>\d{2})\D*(?P<d>\d{2})").unwrap();
    let caps = re.captures(date).unwrap();
    Date {
        year: caps["y"].to_string(),
        month: caps["m"].to_string(),
        day: caps["d"].to_string()
    }
}

fn main() {
    let mut date1 = String::new();
    let mut date2 = String::new();
    print!("Date 1: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut date1).ok().expect("Failed to read Date 1!");

    print!("Date 2: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut date2).ok().expect("Failed to read Date 2!");

    let d1 = to_date(&date1);
    let d2 = to_date(&date2);

    if d1 == d2 {
        println!("Dates are equal");
    } else {
        println!("Dates are not equal");
    }
}
