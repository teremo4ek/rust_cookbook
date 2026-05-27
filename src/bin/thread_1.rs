use std::thread;

fn main() {
    let s = String::from("привет");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("длина: {}", s.len());
        });
    });
}
