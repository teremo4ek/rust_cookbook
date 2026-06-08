use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Привязываем обработчик к адресу
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // Второй элемент содержит IP и порт нового подключения
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // `Connection` позволяет читать/писать кадры (frames) redis вместо
    // потоков байтов. Тип `Connection` определяется mini-redis
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Отвечаем ошибкой
        let response = Frame::Error("Unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
