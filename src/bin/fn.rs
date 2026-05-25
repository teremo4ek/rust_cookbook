fn make_greeter(prefix: String) -> impl Fn(&str) {
    move |name| println!("{} {}", prefix, name)
}

fn main() {
    let hi = make_greeter("Hello".to_string());
    hi("ALL");
}
