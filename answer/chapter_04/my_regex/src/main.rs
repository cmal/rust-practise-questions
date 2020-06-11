// use std::io;
// use std::io::prelude::*;
// use regex::Regex;

// fn main() {

//     let mut input = String::new();
//     print!("Numbers to sort: ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut input).ok().expect("failed to read line");

//     let re = Regex::new(r"\s+").unwrap();
//     let mut vec = Vec::<i32>::new();
//     let numbers: Vec<&str> = re.split(input.trim()).collect();

//     for n in numbers.iter() {
//         vec.push(n.parse().expect("Please enter numbers!"));
//     }

//     vec.sort();
//     println!("sorted: {:?}", vec);
// }


// let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
// let text = "2012-03-14, 2013-01-01 and 2014-07-05";
// for cap in re.captures_iter(text) {
//     println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
// }


// let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
// let before = "2012-03-14, 2013-01-01 and 2014-07-05";
// let after = re.replace_all(before, "$m/$d/$y");
// assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");

use regex::Regex;

#[derive(Debug)]
struct MyRegex {
    year: u32,
    month: u32,
    day: u32
}

fn to_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

fn main() {
    let text = "It was on 2019-03-14, almost after a year from 2018-02-11";
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for cap in re.captures_iter(text) {
        println!("{:?}", MyRegex {
            year: to_u32(&cap[1]),
            month: to_u32(&cap[2]),
            day: to_u32(&cap[3])
        });
    }
}
