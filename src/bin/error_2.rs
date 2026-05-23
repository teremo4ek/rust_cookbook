use anyhow::{anyhow, Context, Result};

fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
    let mut body = response.text()?;
    body.pop();
    body.parse::<u32>()
        .with_context(|| anyhow!("Unexpected response: {}", body))
}

fn run() -> Result<()> {
    let url =
        "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain".to_string();
    let response = reqwest::blocking::get(&url)?;
    let random_value: u32 = parse_response(response)?;
    println!("Random integer between 0 and 10: {}", random_value);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        if let Some(_err) = error.downcast_ref::<std::io::Error>() {
            println!("Standard I/O error: {:?}", error);
        } else if let Some(_err) = error.downcast_ref::<reqwest::Error>() {
            println!("Reqwest error: {:?}", error);
        } else if let Some(_err) = error.downcast_ref::<std::num::ParseIntError>() {
            println!("Parse int error: {:?}", error);
        } else {
            println!("Other error: {:?}", error);
        }
    }
}
