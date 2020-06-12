use std::fmt::Display;
use std::marker::Sized;

struct Table<T: ?Sized> {
    legs_info: T
}

fn show(tb: &Table<dyn Display>) {
    println!("{}", &tb.legs_info);
}

fn main() {

    let tb1 = Table {
        legs_info: "Work in progress...".to_string()
    };

    let tb2 = Table {
        legs_info: 4usize
    };

    show(&tb1);
    show(&tb2);
}
