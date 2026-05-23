struct Dog {
    name: String,
    #[allow(dead_code)]
    age: i8,
}
struct Cat {
    #[allow(dead_code)]
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Какая милаха! Как тебя зовут? {}", self.talk());
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Гав, меня зовут {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Мау!")
    }
}

fn main() {
    let captain_floof = Cat { lives: 9 };
    let fido = Dog {
        name: String::from("Фидо"),
        age: 5,
    };

    captain_floof.greet();
    fido.greet();

    println!(
        "{} {}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    println!(
        "{} {}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}
