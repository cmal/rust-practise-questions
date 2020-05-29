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

// struct ParseError {}

// type DateResult = Result<Date, ParseError>;

fn to_date(date: &str) -> Date {
    let re = Regex::new(r"(?P<y>\d{4})\D*(?P<m>\d{2})\D*(?P<d>\d{2})").unwrap();

    let caps = re.captures(date).unwrap();
    Date {
        year: caps["y"].to_string(),
        month: caps["m"].to_string(),
        day: caps["d"].to_string()
    }
    // let digit = Regex::new(r"\d").unwrap();
    // let nonDigits = digit.replace_all(&date, |caps: &Captures| {
    //     caps.at(1).unwrap().to_owned()
    // });
    // let mut date1 = "";
    // if nonDigits.len() > 0 {
    //     date1 = Regex::new(&nonDigits[0..1]).replace_all(&date);
    // } else {
    //     date1 = date;
    // }

    // if date.len() != 6 {
    //     Err(ParseError{})
    // } else {
    //     Ok(Date {
    //         year: date1[0..4].to_string(),
    //         month: date1[4..6].to_string(),
    //         day: date[6..].to_string()
    //     })
    // }

    // let re = Regex::new(r"\d+").unwrap();
    // re.replace_all(&date, |caps: &Captures| {
    //     Date {
    //         year: caps.at(0).unwrap().to_owned(),
    //         month: caps.at(1).unwrap().to_owned(),
    //         day: caps.at(2).unwrap().to_owned()
    //     }
    // });
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
