use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use std::fmt;

#[derive(Debug, Deserialize)]
struct Rgb {
    red: u8,
    blue: u8,
    green: u8,
}

impl Rgb {
    fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
        let color: Rgb = csv::Reader::from_reader(csv_data)
            .deserialize()
            .nth(0)
            .ok_or_else(|| anyhow!("Cannot parse first CSV record"))?
            .context("Cannot parse RGB color")?;

        Ok(color)
    }
}

impl fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hexa = u32::from(self.red) << 16 | u32::from(self.blue) << 8 | u32::from(self.green);
        write!(f, "{:X}", hexa)
    }
}

fn run() -> Result<()> {
    let csv = "red,blue,green
102,256,204";

    let rgb = Rgb::from_reader(csv.as_bytes()).context("Cannot read CSV data")?;
    println!("{:?} in hexadecimal format: #{:X}", rgb, rgb);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        let chain = err.chain().enumerate().collect::<Vec<_>>();
        if chain.len() > 1 {
            eprintln!("Error chain:");
            chain.iter().for_each(|(i, e)| eprintln!("  {}> {}", i, e));
        } else {
            eprintln!("Error: {:#}", err);
        }
    }
}
