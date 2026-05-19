use anyhow::{bail, Result};
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use std::fs::File;
use std::str::FromStr;

struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
        if buffer_size == 0 {
            bail!("invalid `buffer_size`, must be greater than 0");
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;

            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);

            Some(
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).unwrap(),
            )
        }
    }
}

fn main() -> Result<()> {
    let url = "https://httpbin.org/range/102400?duration=2";
    const CHUNK_SIZE: u32 = 10240;

    let client = reqwest::blocking::Client::new();
    let response = client.head(url).send()?;
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or(anyhow::anyhow!("response does not contain content length"))?;
    let length =
        u64::from_str(length.to_str()?).map_err(|_| anyhow::anyhow!("invalid `Content-Length` header"))?;

    let mut output_file = File::create("output/download.bin")?;

    println!("starting download...");
    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
        println!("range {:?}", range);
        let mut response = client.get(url).header(RANGE, range).send()?;

        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            bail!("unexpected server response: {}", status)
        }
        std::io::copy(&mut response, &mut output_file)?;
    }

    // Read response as text
    let content = response.text()?;
    // Copy response bytes to file
    std::io::copy(&mut content.as_bytes(), &mut output_file)?;

    println!("download completed successfully");

    Ok(())
}
