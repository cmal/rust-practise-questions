fn accept_closure<F>(f: F) -> String
where F: FnOnce() -> String {
    f()
}

fn main() {
    let closure = || "xxxx".to_string();
    println!("{}", accept_closure(closure));
}
