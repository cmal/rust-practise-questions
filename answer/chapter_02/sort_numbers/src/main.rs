use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {

    let mut input = String::new();
    print!("Numbers to sort: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok().expect("failed to read line");

    let re = Regex::new(r"\s+").unwrap();
    let mut vec = Vec::<i32>::new();
    let numbers: Vec<&str> = re.split(input.trim()).collect();

    for n in numbers.iter() {
        vec.push(n.parse().expect("Please enter numbers!"));
    }

    vec.sort();
    println!("sorted: {:?}", vec);
}
