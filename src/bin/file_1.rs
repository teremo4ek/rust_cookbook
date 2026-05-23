use same_file::Handle;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() -> Result<(), Error> {
    let path_to_read = Path::new("output/message.txt");

    let stdout_handle = Handle::stdout()?;
    let handle = Handle::from_path(path_to_read)?;

    if stdout_handle == handle {
        return Err(Error::other(
            "Вы читаете и пишете в один и тот же файл",
        ));
    } else {
        let file = File::open(path_to_read)?;
        let file = BufReader::new(file);
        for (num, line) in file.lines().enumerate() {
            println!("{} : {}", num, line?.to_uppercase());
        }
    }

    Ok(())
}
