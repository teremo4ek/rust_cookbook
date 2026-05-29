use tokio::sync::{mpsc, oneshot};
use tokio::task::spawn;
use tokio::time::{sleep, Duration};

// Рабочая единица. В данном случае она просто спит в течение определенного времени
// и отвечает сообщением в канал `respond_on`
#[derive(Debug)]
struct Work {
    input: u32,
    respond_on: oneshot::Sender<u32>,
}

// Воркер, который ищет работу в очереди (queue) и выполняет ее
async fn worker(mut work_queue: mpsc::Receiver<Work>) {
    let mut iterations = 0;
    let mut timeout_fut = Box::pin(sleep(Duration::from_millis(100)));

    loop {
        tokio::select! {
            _ = &mut timeout_fut => {
                println!("loop timeout iterations: {}", iterations);

                timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
            },

            Some(work) = work_queue.recv() => {
                sleep(Duration::from_millis(10)).await; // выполняем "работу"
                work.respond_on
                    .send(work.input * 1000)
                    .expect("провал отправки ответа");
                iterations += 1;
            }
        }
    }
}

// "Запрашиватель" (requester), который запрашивает работу и ждет ее выполнения
async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
    let (tx, rx) = oneshot::channel();
    work_queue
        .send(Work {
            input,
            respond_on: tx,
        })
        .await
        .expect("провал отправки работы в очередь");
    rx.await.expect("провал ожидания ответа")
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);
    spawn(worker(rx));
    for i in 0..100 {
        let resp = do_work(&tx, i).await;
        println!("результат работы для итерации {i}: {resp}");
    }
}
