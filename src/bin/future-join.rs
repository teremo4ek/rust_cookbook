use anyhow::Result;
use futures::future;
use reqwest;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

async fn size_of_page(url: &str) -> Result<usize> {
    println!("Начинаю загрузку: {}", url);
    let resp = reqwest::get(url).await?;
    let size = resp.text().await?.len();
    println!("Закончил загрузку: {} ({} байт)", url, size);
    Ok(size)
}

async fn fake_db_query(query: &str) -> Result<String> {
    println!("Запрос к БД: {}", query);
    sleep(Duration::from_millis(500)).await; // Симуляция работы БД
    let result = format!("Результат запроса '{}'", query);
    println!("БД ответила");
    Ok(result)
}

#[tokio::main]
async fn main() {
    let urls = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
    ];

    // Создаём фьючерсы для HTTP-запросов
    let http_futures = urls.iter().map(|url| size_of_page(url));

    // Запускаем ВСЕ HTTP-запросы конкурентно через join_all
    let http_results = future::join_all(http_futures).await;

    // А теперь комбинируем разные типы фьючерсов через join!
    // Представьте, что после получения HTTP-ответов нам нужно:
    // 1. Сделать ещё один HTTP-запрос
    // 2. Сходить в базу данных
    // 3. Подождать немного (демонстрация sleep)

    let additional_http = size_of_page("https://www.rust-lang.org");
    let db_query = fake_db_query("SELECT * FROM pages");
    let wait_a_bit = sleep(Duration::from_millis(200));

    // join! ждёт ВСЕ фьючерсы и возвращает кортеж результатов
    let (http_result, db_result, _) = tokio::join!(additional_http, db_query, wait_a_bit);

    // Собираем результаты первого этапа
    let page_sizes_dict: HashMap<&str, Result<usize>> = urls
        .iter()
        .zip(http_results.into_iter())
        .map(|(&url, result)| (url, result))
        .collect();

    println!("\n📊 Результаты HTTP-запросов:");
    for (url, size) in &page_sizes_dict {
        match size {
            Ok(s) => println!("  {}: {} байт", url, s),
            Err(e) => println!("  {}: Ошибка - {}", url, e),
        }
    }

    println!("\n🎯 Результаты второго этапа (join!):");
    match http_result {
        Ok(size) => println!("  Дополнительный HTTP: {} байт", size),
        Err(e) => println!("  Дополнительный HTTP: ошибка - {}", e),
    }

    match db_result {
        Ok(data) => println!("  Запрос к БД: {}", data),
        Err(e) => println!("  Запрос к БД: ошибка - {}", e),
    }

    println!("  Пауза 200мc: выполнена");
}
