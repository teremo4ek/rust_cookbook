use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    ws_stream
        .send(Message::text(
            "Добро пожаловать в чат! Отправьте сообщение".to_string(),
        ))
        .await?;
    let mut bcast_rx = bcast_tx.subscribe();

    // Бесконечный цикл для параллельного выполнения двух задач:
    // 1) получение сообщений из `ws_stream` и их передача клиентам
    // 2) получение сообщений в `bcast_rx` и их отправка клиенту
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("{addr:?}: {text:?}");
                            bcast_tx.send(text.into())?;
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
            msg = bcast_rx.recv() => {
                println!(" bcast_rx.recv() {:?}", msg);
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("Запросы принимаются на порту 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Запрос от {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            let (_, ws_stream) = ServerBuilder::new()
                .accept(socket)
                .await
                .expect("Ошибка при принятии WebSocket");

            if let Err(e) = handle_connection(addr, ws_stream, bcast_tx).await {
                eprintln!("Ошибка в соединении c {addr}: {e}");
            }
        });
    }
}
