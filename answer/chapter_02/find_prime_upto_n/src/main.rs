use std::io;
use std::io::prelude::*; 

fn find_primes(primes: &mut Vec<i32>) {
    primes.retain(|&x| x > 1);

    let mut i = 2;
    while i < *primes.last().unwrap() {
        primes.retain(|&x| x == i || x % i != 0);
        i = *primes.iter().find(|&x| *x > i).unwrap();
    }
}

fn main() {

    let mut input = String::new();
    print!("Find primes upto: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok().expect("failed to read line");
    let n: i32 = input.trim().parse().expect("Please type a number!");

    let mut primes: Vec<i32> = (1..=n).collect();

    find_primes(&mut primes);

    println!("The primes upto {} are {:?}, count: {}", n, primes, primes.len());
}
