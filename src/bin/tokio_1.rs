use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Значение счетчика в задаче: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(count_to(10));
    //count_to(10).await;

    for i in 1..5 {
        println!("Значение счетчика в основной задаче: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }

    handle.await.unwrap();
}
