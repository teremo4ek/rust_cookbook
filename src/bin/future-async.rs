use futures::future::join_all;
use std::time::Instant;
use tokio;

async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
    //std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    let _ = tokio::time::sleep(std::time::Duration::from_millis(duration_ms));
    println!(
        "фьючерс {id} спал в течение {duration_ms} мс, завершился после {} мс",
        start.elapsed().as_millis()
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t * 10));
    join_all(sleep_futures).await;

    println!("app work {}ms", start.elapsed().as_millis());
}
