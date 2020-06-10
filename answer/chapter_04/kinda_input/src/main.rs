use std::io;

#[derive(Debug)]
enum Input {
    Number,
    Letters,
    Symbol
}

fn kinda(s: String) -> Result<Input, String> {
    if s.len() < 1 {
        Err("Empty String".to_string())
    } else {
        match s.parse::<i32>() {
            Ok(_) => Ok(Input::Number),
            Err(_) => if s.chars().all(char::is_alphabetic) {
                Ok(Input::Letters)
            } else {
                // TODO
                Ok(Input::Symbol)
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");

    println!("You typed a kind of {:?}", kinda(input.trim().to_string()));
}
