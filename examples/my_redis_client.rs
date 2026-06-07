use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Подключаемся к серверу mini-redis
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Устанавливаем ключ "hello" в значение "world"
    client.set("hello", "world".into()).await?;

    // Получаем значение ключа "hello"
    let result = client.get("hello").await?;

    println!("От сервера получено: {:?}", result);

    Ok(())
}
