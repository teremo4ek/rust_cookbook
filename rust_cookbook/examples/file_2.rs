use anyhow::{Context, Result};

use std::{env, fs};

fn main() -> Result<()> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    println!(
        "Files modified in the last 24 hours in {}:",
        current_dir.display()
    );

    for entry in fs::read_dir(&current_dir).context("Failed to read directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        let metadata = fs::metadata(&path).context("Failed to read metadata")?;
        let last_modified = metadata
            .modified()
            .context("Failed to get modification time")?
            .elapsed()
            .context("Modification time is in the future")?
            .as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            let file_name = path.file_name().context("Path has no file name")?;
            println!(
                "File: {}, last modified {} seconds ago, read-only: {}, size: {} bytes.",
                file_name.display(),
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
            );
        }
    }

    Ok(())
}
