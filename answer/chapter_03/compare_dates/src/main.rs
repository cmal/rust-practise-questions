use std::io;
use std::io::prelude::*; 
use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
struct Date {
    year: String,
    month: String,
    day: String
}

fn to_date(date: &str) -> Date {
    let re = Regex::new(r"[-/ ]").unwrap();
    let mut iter = re.split(date.trim());
    
    Date {
        year: iter.next().unwrap().to_string(),
        month: iter.next().unwrap().to_string(),
        day: iter.next().unwrap().to_string()
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
