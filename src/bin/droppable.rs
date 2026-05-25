struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("уничтожение {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("выход из блока B");
        }
        println!("выход из блока A");
    }
    drop(a);
    println!("выход из main");
}
