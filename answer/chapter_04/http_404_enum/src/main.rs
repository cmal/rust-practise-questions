#[derive(Debug)]
enum HTTPStatusCode {
    NotFound = 404,
}

fn main() {
    println!("{:?} = {}",
             HTTPStatusCode::NotFound,
             HTTPStatusCode::NotFound as isize,
    );
}
