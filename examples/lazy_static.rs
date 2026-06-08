use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<(), String> {
    let mut db = FRUIT.lock().map_err(|e| e.to_string())?;
    db.push(fruit.to_string());
    Ok(())
}

fn main() -> Result<(), String> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|e| e.to_string())?;

        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("grape")?;
    Ok(())
}
