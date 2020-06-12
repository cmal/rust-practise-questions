use std::thread;

fn main() {
    let th1 = thread::spawn(move || {
        loop {
            println!("Hello");
        }
    });

    let th2 = thread::spawn(move || loop {
        println!("World");
    });
    th1.join().expect("The th1 thread has panicked");
    th2.join().expect("The th2 thread has panicked");
}
