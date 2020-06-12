trait Hello {
    fn say(&self) where Self: std::fmt::Display {
        println!("Hello {}", self);
    }
}

impl Hello for str {}
impl Hello for String {}

fn main() {
    "world!".say();
    "John!".to_string().say();
}
