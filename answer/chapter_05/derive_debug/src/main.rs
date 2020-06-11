#[derive(Debug)]
struct Info<'a> {
    name: &'a str,
    class: &'a str,
    roll: &'a str
}

fn main() {
    println!("{:?}", Info {
        name: &"xxx",
        class: &"yyy",
        roll: &"zzz"
    });
}
