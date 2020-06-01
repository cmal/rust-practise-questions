#[derive(Debug)]
struct MyStruct {
    name: &'static str
}

impl MyStruct {
    fn new(name: &'static str) -> Self {
        MyStruct {
            name: name
        }
    }
}


fn main() {
    println!("{:?}", MyStruct::new("Doggie"));
}
