use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Сообщение {i}")).unwrap();
            println!("{thread_id:?}: отправил сообщение {i}");
        }
        println!("{thread_id:?}: готово");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Основной поток: получено {msg}");
        thread::sleep(Duration::from_millis(10));
    }
}
