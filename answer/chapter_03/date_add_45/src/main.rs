use chrono::offset::Local;
use chrono::{Duration, Datelike};
// use chrono::Date;
// use chrono::prelude::*;  // if we use chrono::Date, must add this, otherwise can not compile

#[derive(Debug)]
struct Date {
    year: i32,
    month: u32,
    day: u32
}

fn main() {
    // cannot declare type Date<Local> here,
    // otherwise will need use chrono::Date,
    // which conflicts with struct Date.
    let today = Local::today();

    println!("{:?}", Date {
        year: today.year(),
        month: today.month(),
        day: today.day()
    });

    println!("Date of 45 days after: {:?}",
             (today + Duration::days(45)).format("%Y %m %d").to_string());
}
