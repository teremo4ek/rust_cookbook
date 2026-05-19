use anyhow::{Context, Result};

use std::fs::File;
use std::io::Read;

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    let first_part = uptime
        .split('.')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Невозможно разобрать данные"))?;

    Ok(first_part.parse()?)
}

fn main() {
    match read_uptime().context("не удалось прочитать uptime") {
        Ok(uptime) => println!("Время безотказной работы: {} секунд", uptime),
        Err(err) => eprintln!("Ошибка: {:#}", err),
    };
}
